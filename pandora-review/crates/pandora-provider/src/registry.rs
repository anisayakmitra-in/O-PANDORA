use std::collections::HashMap;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::provider::Provider;

use crate::types::{ModelCapabilities, ProviderError};

pub struct ProviderRegistry {
    providers: Arc<RwLock<HashMap<String, Arc<dyn Provider>>>>,
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, name: impl Into<String>, provider: Arc<dyn Provider>) {
        let mut providers = self.providers.write().await;

        providers.insert(name.into(), provider);
    }

    pub async fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        let providers = self.providers.read().await;

        providers.get(name).cloned()
    }

    pub async fn list(&self) -> Vec<String> {
        let providers = self.providers.read().await;

        providers.keys().cloned().collect()
    }

    pub async fn capabilities(&self, name: &str) -> Result<ModelCapabilities, ProviderError> {
        let providers = self.providers.read().await;

        let provider = providers.get(name).ok_or_else(|| {
            ProviderError::ProviderUnavailable(format!("provider '{}' not found", name))
        })?;

        Ok(provider.capabilities())
    }
}
