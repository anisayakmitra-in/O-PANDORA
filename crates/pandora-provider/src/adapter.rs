//! Adapter that wraps the async Provider trait into the sync
//! ProviderService contract for the Service Registry.

use pandora_types::services::{ProviderService, Service, ServiceId};
use tokio::runtime::Runtime;

use crate::types::GenerationRequest;

/// Wraps an async Provider into the sync ProviderService trait
/// by running async calls on a tokio runtime.
#[derive(Debug)]
#[allow(dead_code)]
pub struct ProviderServiceAdapter {
    provider_name: String,
    rt: tokio::runtime::Runtime,
}

impl ProviderServiceAdapter {
    pub fn new(provider_name: impl Into<String>) -> Self {
        Self {
            provider_name: provider_name.into(),
            rt: Runtime::new().expect("failed to create tokio runtime"),
        }
    }
}

impl Service for ProviderServiceAdapter {
    fn service_id(&self) -> ServiceId {
        ServiceId::Provider
    }
    fn provider_name(&self) -> &str {
        &self.provider_name
    }
    fn version(&self) -> &str {
        "0.1.0"
    }
}

impl ProviderService for ProviderServiceAdapter {
    fn list_models(&self) -> Result<Vec<String>, String> {
        // Query the ProviderRegistry for available models
        // This is a simplified version - real impl queries each provider
        Ok(vec![])
    }

    fn health(&self) -> Result<String, String> {
        Ok("ok".to_string())
    }

    fn context_limit(&self, _model: &str) -> Result<usize, String> {
        Ok(4096)
    }

    fn cost(&self, _model: &str) -> Result<f64, String> {
        Ok(0.0)
    }

    fn latency(&self, _model: &str) -> Result<f64, String> {
        Ok(0.0)
    }

    fn invoke(&self, model: &str, prompt: &str) -> Result<String, String> {
        let _req = GenerationRequest {
            prompt: prompt.to_string(),
            model: model.to_string(),
            temperature: 0.7,
            max_tokens: 2048,
        };
        let _cancel = tokio_util::sync::CancellationToken::new();
        // In a full implementation, we'd look up the right provider
        // from the registry and call it here
        Err("not implemented: use ProviderRegistry directly".to_string())
    }

    fn supports_tools(&self) -> bool {
        false
    }
    fn supports_images(&self) -> bool {
        false
    }
    fn supports_reasoning(&self) -> bool {
        false
    }
}
