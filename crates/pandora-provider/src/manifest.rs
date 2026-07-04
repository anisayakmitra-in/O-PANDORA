use crate::target::Locality;
use serde::{Deserialize, Serialize};

use crate::capability::ModelCapabilities;

/// Metadata describing a model provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderManifest {
    /// Unique provider identifier (e.g., "ollama", "openrouter", "anthropic").
    pub id: String,

    /// Human-readable provider name.
    pub name: String,

    /// Provider version.
    pub version: String,

    /// Supported model identifiers.
    pub models: Vec<String>,

    /// Provider capabilities.
    pub capabilities: ModelCapabilities,

    /// Provider endpoint/configuration (optional).
    pub endpoint: Option<String>,

    /// Where execution occurs — Local, Remote, or Any.
    pub locality: Locality,
}

impl ProviderManifest {
    /// Create a new provider manifest.
    pub fn new(id: impl Into<String>, name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            models: Vec::new(),
            capabilities: ModelCapabilities::default(),
            endpoint: None,
            locality: Locality::Any,
        }
    }

    /// Add a supported model.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.models.push(model.into());
        self
    }

    /// Set provider capabilities.
    pub fn with_capabilities(mut self, capabilities: ModelCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Set provider endpoint.
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// Set provider locality.
    pub fn with_locality(mut self, locality: Locality) -> Self {
        self.locality = locality;
        self
    }
}
