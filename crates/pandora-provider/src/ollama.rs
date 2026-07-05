use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::capability::ModelCapabilities;
use crate::error::ProviderError;
use crate::manifest::ProviderManifest;
use crate::traits::Provider;
use crate::types::{GenerationRequest, GenerationResponse, TokenChunk};

/// Ollama provider — connects to a local Ollama instance.
pub struct OllamaProvider {
    endpoint: String,
    client: reqwest::Client,
}

impl OllamaProvider {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            client: reqwest::Client::new(),
        }
    }

    pub fn new_default() -> Self {
        // ponytail: check OLLAMA_HOST env var, fall back to default
        let endpoint =
            std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());
        let endpoint = if endpoint.starts_with("http") {
            endpoint
        } else {
            format!("http://{}", endpoint)
        };
        Self::new(endpoint)
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    fn name(&self) -> &'static str {
        "ollama"
    }

    fn manifest(&self) -> ProviderManifest {
        ProviderManifest::new("ollama", "Ollama", "0.1.0")
    }

    async fn generate(
        &self,
        request: GenerationRequest,
        _cancel: CancellationToken,
    ) -> Result<GenerationResponse, ProviderError> {
        let url = format!("{}/api/generate", self.endpoint);
        let body = serde_json::json!({
            "model": request.model,
            "prompt": request.prompt,
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "stream": false,
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                ProviderError::ProviderUnavailable(format!(
                    "Ollama at {}: {} — is it running? Try: OLLAMA_HOST={} ollama serve",
                    self.endpoint, e, self.endpoint
                ))
            })?;

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;

        let text = result["response"].as_str().unwrap_or("").to_string();
        Ok(GenerationResponse {
            text,
            tokens_used: result["eval_count"].as_u64().unwrap_or(0) as usize,
            finish_reason: "stop".to_string(),
        })
    }

    async fn stream_generate(
        &self,
        request: GenerationRequest,
        cancel: CancellationToken,
        tx: mpsc::Sender<TokenChunk>,
    ) -> Result<(), ProviderError> {
        let url = format!("{}/api/generate", self.endpoint);
        let body = serde_json::json!({
            "model": request.model,
            "prompt": request.prompt,
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "stream": true,
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::ProviderUnavailable(e.to_string()))?;

        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        while let Some(chunk) = stream.next().await {
            if cancel.is_cancelled() {
                break;
            }
            let chunk = chunk.map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;
            if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&chunk) {
                if let Some(text) = json["response"].as_str() {
                    let _ = tx
                        .send(TokenChunk {
                            text: text.to_string(),
                        })
                        .await;
                }
            }
        }
        Ok(())
    }

    async fn embed(
        &self,
        text: String,
        _cancel: CancellationToken,
    ) -> Result<Vec<f32>, ProviderError> {
        let url = format!("{}/api/embeddings", self.endpoint);
        let body = serde_json::json!({ "model": "nomic-embed-text", "prompt": text });
        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::ProviderUnavailable(e.to_string()))?;
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;
        let embedding: Vec<f32> = result["embedding"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_f64().map(|f| f as f32))
                    .collect()
            })
            .unwrap_or_default();
        Ok(embedding)
    }

    fn capabilities(&self) -> ModelCapabilities {
        let mut caps = ModelCapabilities::default();
        caps.supports_streaming = true;
        caps.context_window = 8192;
        caps
    }
}
