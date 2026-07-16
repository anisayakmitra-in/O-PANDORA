//! Compatibility Matrix — declared requirements for packages.
//!
//! Every package declares what Pandora version, OS, architecture, permissions,
//! providers, and sandbox level it needs. The runtime validates before install.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompatibilityMatrix {
    pub pandora_version: Option<String>,
    pub os: Vec<String>,
    pub arch: Vec<String>,
    pub permissions: Vec<String>,
    pub sandbox_level: Option<u32>,
    pub max_cost_usd: Option<f64>,
    pub supported_providers: Vec<String>,
    pub tested_models: Vec<String>,
}

impl CompatibilityMatrix {
    pub fn is_compatible(&self) -> bool {
        if !self.os.is_empty() {
            let current_os = if cfg!(target_os = "linux") { "linux" }
            else if cfg!(target_os = "macos") { "macos" }
            else if cfg!(target_os = "windows") { "windows" }
            else { "unknown" };
            if !self.os.iter().any(|o| o == current_os) { return false; }
        }
        if !self.arch.is_empty() {
            let current_arch = if cfg!(target_arch = "x86_64") { "x86_64" }
            else if cfg!(target_arch = "aarch64") { "aarch64" }
            else { "unknown" };
            if !self.arch.iter().any(|a| a == current_arch) { return false; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_matrix_is_compatible() { assert!(CompatibilityMatrix::default().is_compatible()); }
    #[test]
    fn wrong_os_fails() {
        let m = CompatibilityMatrix { os: vec!["nonexistent".into()], ..Default::default() };
        assert!(!m.is_compatible());
    }
}
