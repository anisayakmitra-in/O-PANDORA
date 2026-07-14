//! Package lockfile — reproducible dependency resolution.
//!
//! `pandora.lock` records exact versions of all dependencies.
//! Like Cargo.lock, it ensures reproducible installs across machines.
//!
//! Format:
//! ```toml
//! [packages.pandora/coding-domain]
//! version = "1.4.2"
//! checksum = "sha256:abc123..."
//!
//! [packages.sayak/eda-skill]
//! version = "0.3.1"
//! checksum = "sha256:def456..."
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A locked dependency entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPackage {
    pub version: String,
    pub checksum: String,
    pub source: String,
}

/// The pandora.lock file — exact resolved dependency tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lockfile {
    pub version: u32,
    pub packages: HashMap<String, LockedPackage>,
}

impl Lockfile {
    pub fn new() -> Self { Self { version: 1, packages: HashMap::new() } }
    pub fn add(&mut self, id: &str, version: &str, checksum: &str, source: &str) { self.packages.insert(id.into(), LockedPackage { version: version.into(), checksum: checksum.into(), source: source.into() }); }
    pub fn get(&self, id: &str) -> Option<&LockedPackage> { self.packages.get(id) }
    pub fn has(&self, id: &str) -> bool { self.packages.contains_key(id) }
    pub fn is_empty(&self) -> bool { self.packages.is_empty() }

    /// Read pandora.lock from the current directory.
    pub fn load(path: &str) -> Result<Self, String> { let c = std::fs::read_to_string(path).map_err(|e| format!("Cannot read lockfile: {e}"))?; toml::from_str(&c).map_err(|e| format!("Invalid lockfile: {e}")) }
    /// Write pandora.lock.
    pub fn save(&self, path: &str) -> Result<(), String> { let c = toml::to_string_pretty(self).map_err(|e| format!("Cannot serialize: {e}"))?; std::fs::write(path, c).map_err(|e| format!("Cannot write: {e}")) }
}

impl Default for Lockfile { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn lockfile_add_get() { let mut lf = Lockfile::new(); lf.add("p/a", "1.0", "abc", "palace"); assert!(lf.has("p/a")); }
    #[test] fn lockfile_empty() { assert!(Lockfile::new().is_empty()); }
}
