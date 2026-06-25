use reqwest::blocking::Client;

use serde::{Deserialize, Serialize};

use crate::provider::Provider;

#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    pub model: String,

    pub prompt: String,

    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
}

pub struct OllamaProvider;

impl Provider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    fn infer(&self, model: &str, prompt: &str) -> String {
        let client = Client::new();

        let request = OllamaRequest {
            model: model.to_string(),

            prompt: prompt.to_string(),

            stream: false,
        };

        let response = client
            .post("http://localhost:11434/api/generate")
            .json(&request)
            .send();

        match response {
            Ok(res) => match res.json::<OllamaResponse>() {
                Ok(parsed) => parsed.response,

                Err(_) => String::from("Failed to parse Ollama response."),
            },

            Err(_) => String::from("Failed to connect to Ollama."),
        }
    }
}
