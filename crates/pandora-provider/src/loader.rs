//! Provider config loader. Scans ~/.pandora/providers/ for .toml files.

use std::path::PathBuf;

#[cfg(feature = "legacy-ollama")]
use crate::custom::{load_provider_from_toml, CustomProvider};

/// Default directory for user-defined provider configs.
pub fn providers_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".pandora").join("providers")
}

/// Load all custom providers from the providers directory.
#[cfg(feature = "legacy-ollama")]
pub fn load_custom_providers() -> Vec<CustomProvider> {
    #[cfg(not(feature = "legacy-ollama"))]
    pub fn load_custom_providers() -> Vec<CustomProvider> {
        vec![]
    }

    let dir = providers_dir();
    if !dir.exists() {
        return vec![];
    }

    let mut providers = vec![];
    for entry in std::fs::read_dir(&dir)
        .unwrap_or_else(|_| std::fs::read_dir(".").unwrap())
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "toml") {
            match load_provider_from_toml(path.to_str().unwrap_or("")) {
                Ok(p) => {
                    println!("[PROVIDER] Loaded custom: {} ({})", p.name, p.endpoint);
                    providers.push(p);
                }
                Err(e) => {
                    eprintln!("[PROVIDER] Failed to load {}: {}", path.display(), e);
                }
            }
        }
    }
    providers
}

/// Get the path for a new provider config.
pub fn provider_config_path(name: &str) -> PathBuf {
    providers_dir().join(format!("{}.toml", name))
}
