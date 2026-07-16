//! Provider types — LLM provider abstractions.
//!
//! Every provider backend (Ollama, OpenAI, llama.cpp) implements the
//! `Provider` trait. The orchestrator selects providers and dispatches
//! generation requests through this interface.

use serde::{Deserialize, Serialize};

// ── Core types ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub prompt: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub system: Option<String>,
    pub top_p: f32,
}

impl Default for GenerationRequest {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            model: String::new(),
            temperature: 0.3,
            max_tokens: 4096,
            system: None,
            top_p: 0.9,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub text: String,
    pub tokens_used: usize,
    pub model: String,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderManifest {
    pub name: String,
    pub endpoint: String,
    pub models: Vec<String>,
    pub capabilities: Vec<String>,
    pub locality: String,
}

impl Default for ProviderManifest {
    fn default() -> Self {
        Self {
            name: "ollama".into(),
            endpoint: "http://localhost:11434".into(),
            models: vec![std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "".into())],
            capabilities: vec!["text".into()],
            locality: "local".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTarget {
    pub provider: String,
    pub model: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub locality: String,
}

/// The Provider trait — any LLM backend implements this.
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn generate(&self, request: GenerationRequest) -> Result<String, String>;
    fn manifest(&self) -> ProviderManifest {
        ProviderManifest::default()
    }
}

// ── Ollama provider ──

pub mod ollama {
    use super::*;

    pub struct OllamaProvider {
        pub endpoint: String,
        pub model: String,
    }

    impl OllamaProvider {
        pub fn new(endpoint: &str, model: &str) -> Self {
            Self {
                endpoint: endpoint.to_string(),
                model: model.to_string(),
            }
        }
        pub fn new_default() -> Self {
            Self {
                endpoint: "http://localhost:11434".into(),
                model: std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "".into()),
            }
        }
    }

    impl Provider for OllamaProvider {
        fn name(&self) -> &str {
            "ollama"
        }
        fn manifest(&self) -> ProviderManifest {
            ProviderManifest {
                name: "ollama".into(),
                endpoint: self.endpoint.clone(),
                models: vec![self.model.clone()],
                capabilities: vec!["text".into()],
                locality: "local".into(),
            }
        }
        fn generate(&self, request: GenerationRequest) -> Result<String, String> {
            let url = format!("{}/api/generate", self.endpoint);
            let body = serde_json::json!({
                "model": self.model, "prompt": request.prompt,
                "options": { "temperature": request.temperature, "num_predict": request.max_tokens },
                "stream": false
            });
            let client = reqwest::blocking::Client::new();
            let resp = client
                .post(&url)
                .json(&body)
                .send()
                .map_err(|e| format!("req failed: {e}"))?;
            let json: serde_json::Value = resp.json().map_err(|e| format!("parse failed: {e}"))?;
            Ok(json["response"].as_str().unwrap_or("").to_string())
        }
    }
}

/// Simple cancellation token — replaces tokio_util::sync::CancellationToken.
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct CancellationToken {
    cancelled: Arc<Mutex<bool>>,
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(Mutex::new(false)),
        }
    }
    pub fn cancel(&self) {
        *self.cancelled.lock().unwrap() = true;
    }
    pub fn is_cancelled(&self) -> bool {
        *self.cancelled.lock().unwrap()
    }
}
