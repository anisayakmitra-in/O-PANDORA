//! Configuration file loading — ~/.pandora/config.toml
//!
//! Loaded on startup by PandoraRuntime. Override with env vars.

use crate::permissions_manifest::{PermissionManifest, ShellPermissions};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PandoraConfig {
    /// Default provider name (e.g. "ollama")
    #[serde(default)]
    pub default_provider: Option<String>,
    /// Default model name (e.g. "llama3.2")  
    #[serde(default)]
    pub default_model: Option<String>,
    /// Provider selection policy: "fastest", "cheapest", "default"
    #[serde(default)]
    pub provider_policy: Option<String>,
    /// Maximum execution attempts
    #[serde(default)]
    pub max_attempts: Option<u32>,
    /// Sandbox level: 0=none, 1=restricted, 2=isolated
    #[serde(default)]
    pub sandbox_level: Option<u32>,
    /// Maximum tokens per generation
    #[serde(default)]
    pub max_tokens: Option<usize>,
    /// K-O-Palace registry URL
    #[serde(default)]
    pub registry_url: Option<String>,
    /// Whether to persist events to disk
    #[serde(default)]
    pub persist_events: Option<bool>,
    /// Shell patterns that are always denied for local executions.
    #[serde(default)]
    pub deny_shell_patterns: Vec<String>,
}

impl PandoraConfig {
    /// Load from ~/.pandora/config.toml, falling back to defaults.
    pub fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        } else {
            let cfg = Self::default();
            let _ = cfg.save();
            cfg
        }
    }

    /// Save current config to disk.
    pub fn save(&self) -> Result<(), crate::PandoraError> {
        let dir = config_dir();
        std::fs::create_dir_all(&dir)
            .map_err(|e| crate::PandoraError::Internal(format!("mkdir: {e}")))?;
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| crate::PandoraError::Internal(format!("toml: {e}")))?;
        std::fs::write(Self::path(), toml_str)
            .map_err(|e| crate::PandoraError::Internal(format!("write: {e}")))
    }

    fn path() -> PathBuf {
        config_dir().join("config.toml")
    }

    /// Build the user-level permission manifest used by the runtime.
    pub fn user_permissions(&self) -> PermissionManifest {
        PermissionManifest {
            shell: ShellPermissions {
                enabled: true,
                blocked: self.deny_shell_patterns.clone(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Merge env var overrides. Env vars take precedence over file.
    pub fn with_env(mut self) -> Self {
        if let Ok(v) = std::env::var("PANDORA_DEFAULT_MODEL") {
            if !v.is_empty() {
                self.default_model = Some(v);
            }
        }
        if let Ok(v) = std::env::var("PANDORA_DEFAULT_PROVIDER") {
            if !v.is_empty() {
                self.default_provider = Some(v);
            }
        }
        if let Ok(v) = std::env::var("PANDORA_PROVIDER_POLICY") {
            if !v.is_empty() {
                self.provider_policy = Some(v);
            }
        }
        if let Ok(v) = std::env::var("PANDORA_MAX_ATTEMPTS") {
            if let Ok(n) = v.parse() {
                self.max_attempts = Some(n);
            }
        }
        if let Ok(v) = std::env::var("PANDORA_SANDBOX_LEVEL") {
            if let Ok(n) = v.parse() {
                self.sandbox_level = Some(n);
            }
        }
        if let Ok(v) = std::env::var("PANDORA_MAX_TOKENS") {
            if let Ok(n) = v.parse() {
                self.max_tokens = Some(n);
            }
        }
        self
    }
}

pub fn config_dir() -> PathBuf {
    std::env::var("PANDORA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(home).join(".pandora")
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_defaults() {
        let c = PandoraConfig::default();
        assert!(c.default_model.is_none());
    }
    #[test]
    fn config_with_env() {
        let _guard = crate::test_support::process_env_lock();
        std::env::set_var("PANDORA_DEFAULT_MODEL", "test-model");
        let c = PandoraConfig::default().with_env();
        assert_eq!(c.default_model, Some("test-model".into()));
        std::env::remove_var("PANDORA_DEFAULT_MODEL");
    }
    #[test]
    fn config_dir_exists() {
        let _guard = crate::test_support::process_env_lock();
        let d = config_dir();
        assert!(d.to_string_lossy().contains(".pandora"));
    }
    #[test]
    fn user_permissions_include_persistent_deny_rules() {
        let config = PandoraConfig {
            deny_shell_patterns: vec!["sudo *".into()],
            ..Default::default()
        };
        let permissions = config.user_permissions();
        assert!(matches!(
            permissions.is_shell_allowed("sudo whoami"),
            crate::permissions_manifest::PermissionVerdict::Denied { .. }
        ));
        assert!(matches!(
            permissions.is_shell_allowed("git status"),
            crate::permissions_manifest::PermissionVerdict::Allowed
        ));
    }
}
