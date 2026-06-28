use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::capability::ModelCapabilities;
use crate::error::ProviderError;
use crate::traits::Provider;

/// Registry for managing multiple providers.
#[derive(Default)]
pub struct ProviderRegistry {
    providers: Arc<RwLock<HashMap<String, Arc<dyn Provider>>>>,
}

impl ProviderRegistry {
    /// Create a new empty provider registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a provider (uses `provider.name()` as the key).
    pub async fn register(&self, provider: Arc<dyn Provider>) {
        let mut providers = self.providers.write().await;
        providers.insert(provider.name().to_string(), provider);
    }

    /// Register a provider under a custom name.
    pub async fn register_with_name(&self, name: impl Into<String>, provider: Arc<dyn Provider>) {
        let mut providers = self.providers.write().await;
        providers.insert(name.into(), provider);
    }

    /// Get a provider by name.
    pub async fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        let providers = self.providers.read().await;
        providers.get(name).cloned()
    }

    /// List all registered provider names.
    pub async fn list(&self) -> Vec<String> {
        let providers = self.providers.read().await;
        providers.keys().cloned().collect()
    }

    /// Get a provider manifest by name.
    pub async fn manifest(
        &self,
        name: &str,
    ) -> Result<crate::manifest::ProviderManifest, ProviderError> {
        let providers = self.providers.read().await;
        let provider = providers.get(name).ok_or_else(|| {
            ProviderError::ProviderUnavailable(format!("provider '{name}' not found"))
        })?;
        Ok(provider.manifest())
    }

    /// Get the capabilities of a provider.
    pub async fn capabilities(&self, name: &str) -> Result<ModelCapabilities, ProviderError> {
        let providers = self.providers.read().await;
        let provider = providers.get(name).ok_or_else(|| {
            ProviderError::ProviderUnavailable(format!("provider '{name}' not found"))
        })?;
        Ok(provider.capabilities())
    }

    /// Check whether a provider is registered.
    pub async fn has(&self, name: &str) -> bool {
        let providers = self.providers.read().await;
        providers.contains_key(name)
    }

    /// Remove a provider from the registry.
    pub async fn remove(&self, name: &str) -> bool {
        let mut providers = self.providers.write().await;
        providers.remove(name).is_some()
    }

    /// Get manifests for all registered providers.
    pub async fn all_manifests(&self) -> Vec<crate::manifest::ProviderManifest> {
        let providers = self.providers.read().await;
        providers.values().map(|p| p.manifest()).collect()
    }
}
