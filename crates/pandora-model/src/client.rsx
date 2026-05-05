use reqwest::Client;
use serde_json::json;

use crate::errors::ModelError;
use crate::response::{ChatResponse, Message};

pub struct OllamaClient {
    endpoint: String,
    http_client: Client,
}

impl OllamaClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            http_client: Client::new(),
        }
    }

    pub async fn chat(
        &self,
        model: &str,
        prompt: &str,
    ) -> Result<ChatResponse, ModelError> {
        let url = format!("{}/api/generate", self.endpoint);

        let body = json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        });

        let response = self
            .http_client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| ModelError::Http(e.to_string()))?;

        let text = response
            .text()
            .await
            .map_err(|e| ModelError::Http(e.to_string()))?;

        println!("RAW RESPONSE:\n{}", text);

        // 🔧 TEMP PARSE (since Ollama generate != chat format)
        let parsed: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| ModelError::Http(e.to_string()))?;

        let content = parsed["response"]
            .as_str()
            .unwrap_or("No response content")
            .to_string();

        Ok(ChatResponse {
    model: model.to_string(),
    message: Message {
        role: "assistant".to_string(),
        content,
    },
    done: true,
})
