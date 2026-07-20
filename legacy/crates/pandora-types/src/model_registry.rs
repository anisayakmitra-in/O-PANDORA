//! Dynamic Model Registry — no hardcoded model lists.
//!
//! Models are discovered from provider APIs and cached locally.
//! The registry is refreshable at runtime — no compile-time model lists.
//! Inspired by claurst's models.dev pattern, but simpler: providers report
//! their own models, the registry aggregates them.
//!
//! Invariant: "Never hardcode provider types." This extends to models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

/// A model's capabilities — what it can do.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelCapabilities {
    pub supports_chat: bool,
    pub supports_streaming: bool,
    pub supports_vision: bool,
    pub supports_audio: bool,
    pub supports_tools: bool,
    pub supports_json_mode: bool,
    pub supports_embeddings: bool,
    pub supports_reasoning: bool,
    pub context_window: Option<u32>,
    pub max_output_tokens: Option<u32>,
}

/// A model entry in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    pub id: String,
    pub provider: String,
    pub name: String,
    pub capabilities: ModelCapabilities,
    pub pricing_per_1k_input: Option<f64>,
    pub pricing_per_1k_output: Option<f64>,
    pub discovered_at: SystemTime,
}

/// The registry — aggregates models from all configured connections.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelRegistry {
    pub models: HashMap<String, ModelEntry>,
    pub last_refresh: Option<SystemTime>,
}

impl ModelRegistry {
    pub fn new() -> Self { Self::default() }

    /// Load the cached registry from disk.
    pub fn load() -> Self {
        let path = Self::cache_path();
        if let Ok(data) = std::fs::read_to_string(&path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    /// Save the registry to disk.
    pub fn save(&self) -> Result<(), String> {
        let path = Self::cache_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("mkdir: {e}"))?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| format!("serialize: {e}"))?;
        std::fs::write(&path, json).map_err(|e| format!("write: {e}"))
    }

    /// Register a model discovered from a provider.
    pub fn register(&mut self, model: ModelEntry) {
        self.models.insert(model.id.clone(), model);
    }

    /// Register multiple models from a provider's model list.
    pub fn register_from_provider(&mut self, provider: &str, model_ids: &[String]) {
        for id in model_ids {
            self.register(ModelEntry {
                id: id.clone(),
                provider: provider.into(),
                name: id.clone(),
                capabilities: ModelCapabilities::default(),
                pricing_per_1k_input: None,
                pricing_per_1k_output: None,
                discovered_at: SystemTime::now(),
            });
        }
    }

    /// Find a model by ID.
    pub fn find(&self, id: &str) -> Option<&ModelEntry> {
        self.models.get(id)
    }

    /// List all models from a given provider.
    pub fn by_provider(&self, provider: &str) -> Vec<&ModelEntry> {
        self.models.values().filter(|m| m.provider == provider).collect()
    }

    /// Find models that support a capability.
    pub fn with_capability(&self, cap_fn: impl Fn(&ModelCapabilities) -> bool) -> Vec<&ModelEntry> {
        self.models.values().filter(|m| cap_fn(&m.capabilities)).collect()
    }

    /// Find models that support vision.
    pub fn vision_models(&self) -> Vec<&ModelEntry> {
        self.with_capability(|c| c.supports_vision)
    }

    /// Find models that support tool calling.
    pub fn tool_models(&self) -> Vec<&ModelEntry> {
        self.with_capability(|c| c.supports_tools)
    }

    /// Mark refresh time.
    pub fn mark_refreshed(&mut self) {
        self.last_refresh = Some(SystemTime::now());
    }

    /// Total model count.
    pub fn count(&self) -> usize { self.models.len() }

    fn cache_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        PathBuf::from(home).join(".pandora/model_registry.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_starts_empty() {
        let r = ModelRegistry::new();
        assert_eq!(r.count(), 0);
    }

    #[test]
    fn register_and_find() {
        let mut r = ModelRegistry::new();
        r.register(ModelEntry {
            id: "llama3.2".into(),
            provider: "ollama".into(),
            name: "Llama 3.2".into(),
            capabilities: ModelCapabilities { supports_chat: true, ..Default::default() },
            pricing_per_1k_input: None,
            pricing_per_1k_output: None,
            discovered_at: SystemTime::now(),
        });
        assert_eq!(r.count(), 1);
        assert!(r.find("llama3.2").is_some());
        assert!(r.find("nonexistent").is_none());
    }

    #[test]
    fn filter_by_provider() {
        let mut r = ModelRegistry::new();
        r.register_from_provider("ollama", &["llama3.2".into(), "mistral".into()]);
        r.register_from_provider("openai", &["gpt-4".into()]);
        assert_eq!(r.by_provider("ollama").len(), 2);
        assert_eq!(r.by_provider("openai").len(), 1);
    }

    #[test]
    fn filter_by_capability() {
        let mut r = ModelRegistry::new();
        r.register(ModelEntry {
            id: "gpt-4o".into(),
            provider: "openai".into(),
            name: "GPT-4o".into(),
            capabilities: ModelCapabilities { supports_vision: true, supports_tools: true, ..Default::default() },
            pricing_per_1k_input: None,
            pricing_per_1k_output: None,
            discovered_at: SystemTime::now(),
        });
        r.register(ModelEntry {
            id: "llama3.2".into(),
            provider: "ollama".into(),
            name: "Llama 3.2".into(),
            capabilities: ModelCapabilities { supports_chat: true, ..Default::default() },
            pricing_per_1k_input: None,
            pricing_per_1k_output: None,
            discovered_at: SystemTime::now(),
        });
        assert_eq!(r.vision_models().len(), 1);
        assert_eq!(r.tool_models().len(), 1);
        assert_eq!(r.count(), 2);
    }
}
