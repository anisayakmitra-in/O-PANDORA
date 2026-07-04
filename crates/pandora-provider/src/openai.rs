use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::capability::ModelCapabilities;
use crate::error::ProviderError;
use crate::manifest::ProviderManifest;
use crate::traits::Provider;
use crate::types::{GenerationRequest, GenerationResponse, TokenChunk};

pub struct OpenAIProvider {
    api_key: String,
    endpoint: String,
    client: reqwest::Client,
}

impl OpenAIProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            endpoint: "https://api.openai.com/v1".to_string(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "openai"
    }

    fn manifest(&self) -> ProviderManifest {
        ProviderManifest::new("openai", "OpenAI", "0.1.0")
    }

    async fn generate(
        &self,
        request: GenerationRequest,
        _cancel: CancellationToken,
    ) -> Result<GenerationResponse, ProviderError> {
        let url = format!("{}/chat/completions", self.endpoint);
        let body = serde_json::json!({
            "model": request.model,
            "messages": [{"role": "user", "content": request.prompt}],
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::ProviderUnavailable(e.to_string()))?;

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;

        let text = result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let tokens_used = result["usage"]["total_tokens"].as_u64().unwrap_or(0) as usize;
        Ok(GenerationResponse {
            text,
            tokens_used,
            finish_reason: "stop".to_string(),
        })
    }

    async fn stream_generate(
        &self,
        request: GenerationRequest,
        cancel: CancellationToken,
        tx: mpsc::Sender<TokenChunk>,
    ) -> Result<(), ProviderError> {
        let url = format!("{}/chat/completions", self.endpoint);
        let body = serde_json::json!({
            "model": request.model,
            "messages": [{"role": "user", "content": request.prompt}],
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "stream": true,
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
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
            let line = String::from_utf8_lossy(&chunk);
            for l in line.lines() {
                if let Some(content) = l.strip_prefix("data: ") {
                    if content == "[DONE]" {
                        return Ok(());
                    }
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
                        if let Some(text) = json["choices"][0]["delta"]["content"].as_str() {
                            let _ = tx
                                .send(TokenChunk {
                                    text: text.to_string(),
                                })
                                .await;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn embed(
        &self,
        _text: String,
        _cancel: CancellationToken,
    ) -> Result<Vec<f32>, ProviderError> {
        let url = format!("{}/embeddings", self.endpoint);
        let body = serde_json::json!({"model": "text-embedding-3-small", "input": _text});
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::ProviderUnavailable(e.to_string()))?;
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;
        let embedding: Vec<f32> = result["data"][0]["embedding"]
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
        ModelCapabilities {
            supports_streaming: true,
            ..Default::default()
        }
    }
}
