//! Configuration profiles — named execution configurations.
//!
//! Profiles are TOML files in ~/.pandora/profiles/ or PANDORA_PROFILES_DIR.
//! Each profile bundles provider, strategy, sandbox, goal, and evaluator settings.

use serde::Deserialize;

/// A named execution profile loaded from TOML.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Profile {
    pub provider: Option<String>,
    pub strategy: Option<String>,
    pub sandbox: Option<u8>,
    pub goal: Option<String>,
    pub evaluator: Option<String>,
    pub approval: Option<bool>,
    pub max_attempts: Option<u32>,
}

/// Profiles directory. Returns the path to the profiles directory.
pub fn profiles_dir() -> std::path::PathBuf {
    std::env::var("PANDORA_PROFILES_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            std::path::PathBuf::from(home).join(".pandora").join("profiles")
        })
}

/// Load a profile by name from the profiles directory.
pub fn load_profile(name: &str) -> Result<Profile, String> {
    let path = profiles_dir().join(format!("{}.toml", name));
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Profile '{}' not found ({}): {}", name, path.display(), e))?;
    toml::from_str(&content)
        .map_err(|e| format!("Failed to parse profile '{}': {}", name, e))
}

/// List all available profiles.
pub fn list_profiles() -> Result<Vec<String>, String> {
    let dir = profiles_dir();
    let mut profiles = Vec::new();
    for entry in std::fs::read_dir(&dir).map_err(|e| format!("Cannot read profiles dir {}: {}", dir.display(), e))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "toml") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                profiles.push(name.to_string());
            }
        }
    }
    profiles.sort();
    Ok(profiles)
}
