use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::capability::ModelCapabilities;
use crate::error::ProviderError;
use crate::manifest::ProviderManifest;
use crate::traits::Provider;
use crate::types::{GenerationRequest, GenerationResponse, TokenChunk};

/// A user-defined REST API provider loaded from config.
/// Users define endpoint, headers, auth, and response field paths.
pub struct CustomProvider {
    pub name: String,
    pub endpoint: String,
    pub headers: HashMap<String, String>,
    pub models: Vec<String>,
    client: reqwest::Client,
}

impl CustomProvider {
    pub fn new(
        name: impl Into<String>,
        endpoint: impl Into<String>,
        headers: HashMap<String, String>,
        models: Vec<String>,
    ) -> Self {
        Self {
            name: name.into(),
            endpoint: endpoint.into(),
            headers,
            models,
            client: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Option<Self> {
        let endpoint = std::env::var("PROVIDER_ENDPOINT").ok()?;
        let name = std::env::var("PROVIDER_NAME").unwrap_or_else(|_| "custom".into());
        let model = std::env::var("PROVIDER_MODEL").unwrap_or_else(|_| "default".into());
        let mut headers = std::collections::HashMap::new();
        if let Ok(key) = std::env::var("PROVIDER_API_KEY") {
            headers.insert("Authorization".into(), format!("Bearer {}", key));
        }
        headers.insert("Content-Type".into(), "application/json".into());
        Some(Self {
            name,
            endpoint,
            headers,
            models: vec![model],
            client: reqwest::Client::new(),
        })
    }
}

#[async_trait]
impl Provider for CustomProvider {
    fn name(&self) -> &'static str {
        Box::leak(self.name.clone().into_boxed_str())
    }

    fn manifest(&self) -> ProviderManifest {
        let mut m = ProviderManifest::new(&self.name, &self.name, "0.1.0");
        for model in &self.models {
            m = m.with_model(model.clone());
        }
        m
    }

    async fn generate(
        &self,
        request: GenerationRequest,
        _cancel: CancellationToken,
    ) -> Result<GenerationResponse, ProviderError> {
        let body = serde_json::json!({
            "model": request.model,
            "prompt": request.prompt,
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
        });

        let mut req = self.client.post(&self.endpoint);
        for (k, v) in &self.headers {
            req = req.header(k.as_str(), v.as_str());
        }
        let response = req
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::ProviderUnavailable(e.to_string()))?;

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ProviderError::GenerationFailed(e.to_string()))?;

        let text = result["response"]
            .as_str()
            .or_else(|| result["text"].as_str())
            .or_else(|| result["content"].as_str())
            .or_else(|| result["choices"][0]["text"].as_str())
            .or_else(|| result["choices"][0]["message"]["content"].as_str())
            .unwrap_or("")
            .to_string();
        Ok(GenerationResponse {
            text,
            tokens_used: 0,
            finish_reason: "stop".to_string(),
        })
    }

    async fn stream_generate(
        &self,
        request: GenerationRequest,
        cancel: CancellationToken,
        tx: mpsc::Sender<TokenChunk>,
    ) -> Result<(), ProviderError> {
        let body = serde_json::json!({
            "model": request.model,
            "prompt": request.prompt,
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "stream": true,
        });

        let mut req = self.client.post(&self.endpoint);
        for (k, v) in &self.headers {
            req = req.header(k.as_str(), v.as_str());
        }
        let response = req
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
            if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                let _ = tx.send(TokenChunk { text }).await;
            }
        }
        Ok(())
    }

    async fn embed(
        &self,
        _text: String,
        _cancel: CancellationToken,
    ) -> Result<Vec<f32>, ProviderError> {
        Err(ProviderError::ProviderUnavailable(
            "embedding not supported".to_string(),
        ))
    }

    fn capabilities(&self) -> ModelCapabilities {
        ModelCapabilities::default()
    }
}

/// Load a custom provider from a TOML config file.
pub fn load_provider_from_toml(path: &str) -> Result<CustomProvider, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    let config: toml::Value = content
        .parse()
        .map_err(|e| format!("invalid TOML: {}", e))?;

    let provider_type = config
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("rest");
    if provider_type != "rest" {
        return Err(format!("unsupported provider type: {}", provider_type));
    }

    let name = config
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("custom")
        .to_string();
    let endpoint = config
        .get("endpoint")
        .and_then(|v| v.as_str())
        .ok_or("missing 'endpoint' in provider config")?
        .to_string();

    let mut headers = std::collections::HashMap::new();
    if let Some(h) = config.get("headers").and_then(|v| v.as_table()) {
        for (k, v) in h {
            if let Some(val) = v.as_str() {
                headers.insert(k.clone(), val.to_string());
            }
        }
    }

    let models: Vec<String> = config
        .get("models")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Ok(CustomProvider::new(name, endpoint, headers, models))
}
