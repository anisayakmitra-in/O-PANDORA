//! LlamaCpp provider — connects to a local llama.cpp server
//! (OpenAI-compatible API at http://localhost:8080/v1)

use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::capability::ModelCapabilities;
use crate::error::ProviderError;
use crate::manifest::ProviderManifest;
use crate::traits::Provider;
use crate::types::{GenerationRequest, GenerationResponse, TokenChunk};

pub struct LlamaCppProvider {
    endpoint: String,
    client: reqwest::Client,
}

impl LlamaCppProvider {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            client: reqwest::Client::new(),
        }
    }

    /// Create with default local endpoint, check LLAMA_CPP_HOST env var.
    pub fn new_default() -> Self {
        let endpoint = std::env::var("LLAMA_CPP_HOST")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());
        let endpoint = if endpoint.starts_with("http") { endpoint } else { format!("http://{}", endpoint) };
        Self::new(endpoint)
    }
}

#[async_trait]
impl Provider for LlamaCppProvider {
    fn name(&self) -> &'static str { "llama.cpp" }

    fn manifest(&self) -> ProviderManifest {
        ProviderManifest::new("llama.cpp", "LlamaCpp", "0.1.0")
    }

    async fn generate(
        &self,
        request: GenerationRequest,
        _cancel: CancellationToken,
    ) -> Result<GenerationResponse, ProviderError> {
        let url = format!("{}/v1/chat/completions", self.endpoint);
        let body = serde_json::json!({
            "model": request.model,
            "messages": [{"role": "user", "content": request.prompt}],
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "stream": false,
        });

        let response = self.client.post(&url).json(&body).send().await
            .map_err(|e| ProviderError::ProviderUnavailable(
                format!("llama.cpp at {}: {} — is it running? Try: LLAMA_CPP_HOST={} llama-server", self.endpoint, e, self.endpoint)
            ))?;

        let result: serde_json::Value = response.json().await
            .map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;

        let text = result["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        Ok(GenerationResponse { text, tokens_used: 0, finish_reason: "stop".into() })
    }

    async fn stream_generate(
        &self,
        request: GenerationRequest,
        cancel: CancellationToken,
        tx: mpsc::Sender<TokenChunk>,
    ) -> Result<(), ProviderError> {
        let url = format!("{}/v1/chat/completions", self.endpoint);
        let body = serde_json::json!({
            "model": request.model,
            "messages": [{"role": "user", "content": request.prompt}],
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "stream": true,
        });

        let response = self.client.post(&url).json(&body).send().await
            .map_err(|e| ProviderError::ProviderUnavailable(e.to_string()))?;

        use futures_util::StreamExt;
        let mut byte_stream = response.bytes_stream();
        while let Some(chunk) = byte_stream.next().await {
            if cancel.is_cancelled() { break; }
            let chunk = chunk.map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;
            let text = String::from_utf8_lossy(&chunk);
            for line in text.lines() {
                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" { return Ok(()); }
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            let _ = tx.send(TokenChunk { text: content.to_string() }).await;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn embed(&self, _text: String, _cancel: CancellationToken) -> Result<Vec<f32>, ProviderError> {
        Err(ProviderError::GenerationFailed("embedding not supported by llama.cpp provider".into()))
    }

    fn capabilities(&self) -> ModelCapabilities {
        let mut caps = ModelCapabilities::default();
        caps.supports_streaming = true;
        caps.context_window = 8192;
        caps
    }
}
