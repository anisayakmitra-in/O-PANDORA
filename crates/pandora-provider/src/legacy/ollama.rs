//! TEMPORARY legacy Ollama implementation.
//!
//! This module exists only to keep the original Ollama HTTP client
//! available while the plugin architecture is being designed. It is
//! NOT part of the public provider contract. Do not use it in new
//! code. Enable with the `legacy-ollama` cargo feature.

use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::capability::{LanguageSupport, ModelCapabilities};
use crate::error::ProviderError;
use crate::traits::Provider;
use crate::types::{GenerationRequest, GenerationResponse, TokenChunk};

/// Temporary Ollama HTTP client.
///
/// ⚠️ Will be removed once the plugin architecture lands. Kept only
/// for backward compatibility with code that still depends on the
/// legacy `OllamaProvider` type.
pub struct OllamaProvider {
    client: Client,
    base_url: String,
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OllamaProvider {
    /// Create a new Ollama client pointing at `http://localhost:11434`.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: String::from("http://localhost:11434"),
        }
    }

    /// Create a new Ollama client with a custom base URL.
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    fn name(&self) -> &'static str {
        "ollama"
    }

    fn manifest(&self) -> crate::manifest::ProviderManifest {
        crate::manifest::ProviderManifest::new("ollama", "Ollama", "0.1.0")
            .with_capabilities(self.capabilities())
            .with_endpoint(self.base_url.clone())
    }

    async fn generate(
        &self,
        request: GenerationRequest,
        cancel: CancellationToken,
    ) -> Result<GenerationResponse, ProviderError> {
        if cancel.is_cancelled() {
            return Err(ProviderError::Cancelled);
        }

        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&json!({
                "model": request.model,
                "prompt": request.prompt,
                "stream": false,
                "options": {
                    "temperature": request.temperature,
                    "num_predict": request.max_tokens,
                }
            }))
            .send()
            .await
            .map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;

        let value: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;

        let text = value["response"].as_str().unwrap_or("").to_string();

        Ok(GenerationResponse {
            tokens_used: text.len(),
            text,
            finish_reason: String::from("stop"),
        })
    }

    async fn stream_generate(
        &self,
        request: GenerationRequest,
        cancel: CancellationToken,
        tx: mpsc::Sender<TokenChunk>,
    ) -> Result<(), ProviderError> {
        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&json!({
                "model": request.model,
                "prompt": request.prompt,
                "stream": true,
                "options": {
                    "temperature": request.temperature,
                    "num_predict": request.max_tokens,
                }
            }))
            .send()
            .await
            .map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;

        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            if cancel.is_cancelled() {
                return Err(ProviderError::Cancelled);
            }

            let chunk = chunk_result.map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                let parsed: serde_json::Value = match serde_json::from_str(line) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if let Some(token) = parsed["response"].as_str() {
                    tx.send(TokenChunk {
                        text: token.to_string(),
                    })
                    .await
                    .map_err(|_| {
                        ProviderError::GenerationFailed(String::from("stream channel closed"))
                    })?;
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
        Err(ProviderError::ProviderUnavailable(String::from(
            "embeddings not implemented yet",
        )))
    }

    fn capabilities(&self) -> ModelCapabilities {
        ModelCapabilities {
            multilingual: true,
            supported_languages: vec![
                lang("en", "English", "United States", 0.98),
                lang("bn", "Bengali", "Bangladesh", 0.90),
                lang("hi", "Hindi", "India", 0.92),
                lang("ja", "Japanese", "Japan", 0.88),
                lang("zh", "Mandarin Chinese", "China", 0.94),
                lang("ko", "Korean", "South Korea", 0.87),
                lang("ru", "Russian", "Russia", 0.86),
                lang("de", "German", "Germany", 0.85),
                lang("fr", "French", "France", 0.85),
                lang("es", "Spanish", "Spain", 0.90),
                lang("pt", "Portuguese", "Brazil", 0.84),
                lang("it", "Italian", "Italy", 0.82),
                lang("tr", "Turkish", "Turkey", 0.80),
                lang("id", "Indonesian", "Indonesia", 0.81),
                lang("vi", "Vietnamese", "Vietnam", 0.79),
                lang("th", "Thai", "Thailand", 0.77),
                lang("ar", "Arabic", "Saudi Arabia", 0.83),
                lang("fa", "Persian", "Iran", 0.76),
                lang("pl", "Polish", "Poland", 0.78),
                lang("nl", "Dutch", "Netherlands", 0.77),
                lang("uk", "Ukrainian", "Ukraine", 0.74),
                lang("el", "Greek", "Greece", 0.73),
                lang("he", "Hebrew", "Israel", 0.75),
                lang("sv", "Swedish", "Sweden", 0.72),
                lang("fi", "Finnish", "Finland", 0.70),
                lang("ro", "Romanian", "Romania", 0.71),
                lang("cs", "Czech", "Czech Republic", 0.70),
                lang("hu", "Hungarian", "Hungary", 0.69),
                lang("ur", "Urdu", "Pakistan", 0.82),
                lang("sw", "Swahili", "Kenya", 0.65),
            ],
            context_window: 32768,
            supports_streaming: true,
            supports_embeddings: false,
            supports_tools: false,
        }
    }
}

fn lang(code: &str, name: &str, country: &str, confidence: f32) -> LanguageSupport {
    LanguageSupport {
        language_code: code.into(),
        language_name: name.into(),
        country: country.into(),
        confidence,
    }
}
