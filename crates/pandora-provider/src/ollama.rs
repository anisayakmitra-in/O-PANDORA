use futures_util::StreamExt;

use async_trait::async_trait;

use reqwest::Client;

use serde_json::json;

use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use crate::provider::Provider;

use crate::types::{
    GenerationRequest, GenerationResponse, LanguageSupport, ModelCapabilities, ProviderError,
    TokenChunk,
};

pub struct OllamaProvider {
    client: Client,

    base_url: String,
}

impl OllamaProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),

            base_url: String::from("http://localhost:11434"),
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    fn name(&self) -> &'static str {
        "ollama"
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

                "model":
                    request.model,

                "prompt":
                    request.prompt,

                "stream":
                    false,

                "options": {

                    "temperature":
                        request.temperature,

                    "num_predict":
                        request.max_tokens,
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

                "model":
                    request.model,

                "prompt":
                    request.prompt,

                "stream":
                    true,

                "options": {

                    "temperature":
                        request.temperature,

                    "num_predict":
                        request.max_tokens,
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

                    Err(_) => {
                        continue;
                    }
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
                LanguageSupport {
                    language_code: String::from("en"),

                    language_name: String::from("English"),

                    country: String::from("United States"),

                    confidence: 0.98,
                },
                LanguageSupport {
                    language_code: String::from("bn"),

                    language_name: String::from("Bengali"),

                    country: String::from("Bangladesh"),

                    confidence: 0.90,
                },
                LanguageSupport {
                    language_code: String::from("hi"),

                    language_name: String::from("Hindi"),

                    country: String::from("India"),

                    confidence: 0.92,
                },
                LanguageSupport {
                    language_code: String::from("ja"),

                    language_name: String::from("Japanese"),

                    country: String::from("Japan"),

                    confidence: 0.88,
                },
                LanguageSupport {
                    language_code: String::from("zh"),

                    language_name: String::from("Mandarin Chinese"),

                    country: String::from("China"),

                    confidence: 0.94,
                },
                LanguageSupport {
                    language_code: String::from("ko"),

                    language_name: String::from("Korean"),

                    country: String::from("South Korea"),

                    confidence: 0.87,
                },
                LanguageSupport {
                    language_code: String::from("ru"),

                    language_name: String::from("Russian"),

                    country: String::from("Russia"),

                    confidence: 0.86,
                },
                LanguageSupport {
                    language_code: String::from("de"),

                    language_name: String::from("German"),

                    country: String::from("Germany"),

                    confidence: 0.85,
                },
                LanguageSupport {
                    language_code: String::from("fr"),

                    language_name: String::from("French"),

                    country: String::from("France"),

                    confidence: 0.85,
                },
                LanguageSupport {
                    language_code: String::from("es"),

                    language_name: String::from("Spanish"),

                    country: String::from("Spain"),

                    confidence: 0.90,
                },
                LanguageSupport {
                    language_code: String::from("pt"),

                    language_name: String::from("Portuguese"),

                    country: String::from("Brazil"),

                    confidence: 0.84,
                },
                LanguageSupport {
                    language_code: String::from("it"),

                    language_name: String::from("Italian"),

                    country: String::from("Italy"),

                    confidence: 0.82,
                },
                LanguageSupport {
                    language_code: String::from("tr"),

                    language_name: String::from("Turkish"),

                    country: String::from("Turkey"),

                    confidence: 0.80,
                },
                LanguageSupport {
                    language_code: String::from("id"),

                    language_name: String::from("Indonesian"),

                    country: String::from("Indonesia"),

                    confidence: 0.81,
                },
                LanguageSupport {
                    language_code: String::from("vi"),

                    language_name: String::from("Vietnamese"),

                    country: String::from("Vietnam"),

                    confidence: 0.79,
                },
                LanguageSupport {
                    language_code: String::from("th"),

                    language_name: String::from("Thai"),

                    country: String::from("Thailand"),

                    confidence: 0.77,
                },
                LanguageSupport {
                    language_code: String::from("ar"),

                    language_name: String::from("Arabic"),

                    country: String::from("Saudi Arabia"),

                    confidence: 0.83,
                },
                LanguageSupport {
                    language_code: String::from("fa"),

                    language_name: String::from("Persian"),

                    country: String::from("Iran"),

                    confidence: 0.76,
                },
                LanguageSupport {
                    language_code: String::from("pl"),

                    language_name: String::from("Polish"),

                    country: String::from("Poland"),

                    confidence: 0.78,
                },
                LanguageSupport {
                    language_code: String::from("nl"),

                    language_name: String::from("Dutch"),

                    country: String::from("Netherlands"),

                    confidence: 0.77,
                },
                LanguageSupport {
                    language_code: String::from("uk"),

                    language_name: String::from("Ukrainian"),

                    country: String::from("Ukraine"),

                    confidence: 0.74,
                },
                LanguageSupport {
                    language_code: String::from("el"),

                    language_name: String::from("Greek"),

                    country: String::from("Greece"),

                    confidence: 0.73,
                },
                LanguageSupport {
                    language_code: String::from("he"),

                    language_name: String::from("Hebrew"),

                    country: String::from("Israel"),

                    confidence: 0.75,
                },
                LanguageSupport {
                    language_code: String::from("sv"),

                    language_name: String::from("Swedish"),

                    country: String::from("Sweden"),

                    confidence: 0.72,
                },
                LanguageSupport {
                    language_code: String::from("fi"),

                    language_name: String::from("Finnish"),

                    country: String::from("Finland"),

                    confidence: 0.70,
                },
                LanguageSupport {
                    language_code: String::from("ro"),

                    language_name: String::from("Romanian"),

                    country: String::from("Romania"),

                    confidence: 0.71,
                },
                LanguageSupport {
                    language_code: String::from("cs"),

                    language_name: String::from("Czech"),

                    country: String::from("Czech Republic"),

                    confidence: 0.70,
                },
                LanguageSupport {
                    language_code: String::from("hu"),

                    language_name: String::from("Hungarian"),

                    country: String::from("Hungary"),

                    confidence: 0.69,
                },
                LanguageSupport {
                    language_code: String::from("ur"),

                    language_name: String::from("Urdu"),

                    country: String::from("Pakistan"),

                    confidence: 0.82,
                },
                LanguageSupport {
                    language_code: String::from("sw"),

                    language_name: String::from("Swahili"),

                    country: String::from("Kenya"),

                    confidence: 0.65,
                },
            ],

            context_window: 32768,

            supports_streaming: false,

            supports_embeddings: false,

            supports_tools: false,
        }
    }
}
