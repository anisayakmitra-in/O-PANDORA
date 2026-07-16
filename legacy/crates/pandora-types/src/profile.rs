//! Configuration profiles — named execution configurations.
//!
//! Profiles are TOML files in `~/.pandora/profiles/` or
//! `PANDORA_PROFILES_DIR`. Each profile bundles provider, strategy,
//! sandbox, goal, and evaluator settings into a single named file.

use serde::Deserialize;
use std::path::PathBuf;

/// A named execution profile loaded from TOML.
///
/// All fields are optional — profiles are partial configurations that
/// override defaults rather than replacing them entirely.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Profile {
    /// Provider to use, e.g. "ollama", "openai".
    pub provider: Option<String>,
    /// Control strategy, e.g. "closed", "single".
    pub strategy: Option<String>,
    /// Sandbox level (0 = none, 1 = restricted, 2 = isolated).
    pub sandbox: Option<u8>,
    /// Goal string passed to the evaluator.
    pub goal: Option<String>,
    /// Evaluator to use, e.g. "rust-tests".
    pub evaluator: Option<String>,
    /// Whether to require manual approval.
    pub approval: Option<bool>,
    /// Maximum execution attempts.
    pub max_attempts: Option<u32>,
}

/// Path to the profiles directory.
///
/// Checks `PANDORA_PROFILES_DIR` first, then falls back to
/// `~/.pandora/profiles/`.
pub fn profiles_dir() -> PathBuf {
    std::env::var("PANDORA_PROFILES_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".pandora").join("profiles")
        })
}

/// Load a profile by name.
///
/// Reads `<profiles_dir>/<name>.toml` and deserializes it.
/// Returns a descriptive error if the file is missing or malformed.
pub fn load_profile(name: &str) -> Result<Profile, String> {
    let path = profiles_dir().join(format!("{name}.toml"));
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Profile '{name}' not found ({}): {e}", path.display()))?;
    toml::from_str(&content).map_err(|e| format!("Failed to parse profile '{name}': {e}"))
}

/// List all available profiles (files with `.toml` extension).
pub fn list_profiles() -> Result<Vec<String>, String> {
    let dir = profiles_dir();
    let mut profiles = Vec::new();
    for entry in std::fs::read_dir(&dir)
        .map_err(|e| format!("Cannot read profiles dir {}: {}", dir.display(), e))?
    {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                profiles.push(name.to_string());
            }
        }
    }
    profiles.sort();
    Ok(profiles)
}
