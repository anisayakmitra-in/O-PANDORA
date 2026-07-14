//! Pandora Ollama Provider — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaRequest {
    pub model: String,

    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub response: String,

    pub success: bool,
}

pub struct OllamaProvider;

impl OllamaProvider {
    pub async fn generate(request: &OllamaRequest) -> OllamaResponse {
        println!("[OLLAMA] model={}", request.model);

        let client = Client::new();

        let payload = serde_json::json!({

            "model":
                request.model,

            "prompt":
                request.prompt,

            "stream":
                false
        });

        let result = client
            .post("http://localhost:11434/api/generate")
            .json(&payload)
            .send()
            .await;

        match result {
            Ok(response) => match response.json::<serde_json::Value>().await {
                Ok(json) => {
                    let output = json["response"].as_str().unwrap_or("").to_string();

                    OllamaResponse {
                        response: output,

                        success: true,
                    }
                }

                Err(error) => OllamaResponse {
                    response: error.to_string(),

                    success: false,
                },
            },

            Err(error) => OllamaResponse {
                response: error.to_string(),

                success: false,
            },
        }
    }
}
