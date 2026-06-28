//! Storage backend configuration.

use serde::{Deserialize, Serialize};

use crate::types::BackendKind;

/// Static configuration for a single storage backend instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Human-readable backend name (e.g. `primary`, `audit-archive`).
    pub name: String,

    /// Backend kind, used to select the matching adapter.
    pub kind: BackendKind,

    /// Backend-specific connection string, URI, or path.
    /// Examples:
    /// - `postgres://user:pass@host/db`
    /// - `sqlite://./data/audit.db`
    /// - `/var/lib/pandora/state.jsonl`
    /// - `qdrant://localhost:6334`
    pub endpoint: String,

    /// Optional namespace / key prefix.
    #[serde(default)]
    pub namespace: Option<String>,

    /// Optional pool size. `None` means "backend default".
    #[serde(default)]
    pub pool_size: Option<u32>,

    /// Optional request timeout in seconds. `None` means "no
    /// explicit timeout".
    #[serde(default)]
    pub timeout_secs: Option<u64>,

    /// Free-form backend-specific options.
    #[serde(default)]
    pub options: std::collections::BTreeMap<String, String>,
}

impl StorageConfig {
    /// Create a new config with the given name, kind, and endpoint.
    pub fn new(name: impl Into<String>, kind: BackendKind, endpoint: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind,
            endpoint: endpoint.into(),
            namespace: None,
            pool_size: None,
            timeout_secs: None,
            options: std::collections::BTreeMap::new(),
        }
    }

    /// Override the namespace.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    /// Override the pool size.
    pub fn with_pool_size(mut self, pool_size: u32) -> Self {
        self.pool_size = Some(pool_size);
        self
    }

    /// Override the timeout.
    pub fn with_timeout_secs(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = Some(timeout_secs);
        self
    }

    /// Add a backend-specific option.
    pub fn with_option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }
}

/// Collection of backend configurations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageConfigSet {
    configs: Vec<StorageConfig>,
}

impl StorageConfigSet {
    /// Empty config set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a backend configuration.
    pub fn with(mut self, config: StorageConfig) -> Self {
        self.configs.push(config);
        self
    }

    /// All configured backends.
    pub fn all(&self) -> &[StorageConfig] {
        &self.configs
    }

    /// Look up a backend by name.
    pub fn find(&self, name: &str) -> Option<&StorageConfig> {
        self.configs.iter().find(|c| c.name == name)
    }
}
