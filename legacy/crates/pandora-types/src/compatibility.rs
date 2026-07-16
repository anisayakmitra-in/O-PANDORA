//! Compatibility Matrix — declared requirements for packages.
//!
//! Every package declares what Pandora version, OS, architecture, permissions,
//! providers, and sandbox level it needs. The runtime validates before install.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompatibilityMatrix {
    /// Minimum Pandora version (semver range).
    pub pandora_version: Option<String>,
    /// Supported operating systems.
    pub os: Vec<String>,
    /// Supported architectures.
    pub arch: Vec<String>,
    /// Required permissions (e.g. "network", "filesystem", "sandbox").
    pub permissions: Vec<String>,
    /// Minimum sandbox level required (0=none, 1=restricted, 2=isolated).
    pub sandbox_level: Option<u32>,
    /// Maximum cost per execution in USD.
    pub max_cost_usd: Option<f64>,
    /// Provider kinds this package works with.
    pub supported_providers: Vec<String>,
    /// Models this package has been tested with.
    pub tested_models: Vec<String>,
}

impl CompatibilityMatrix {
    /// Check if the current runtime can run this package.
    pub fn is_compatible(&self) -> bool {
        // OS check
        if !self.os.is_empty() {
            let current_os = if cfg!(target_os = "linux") { "linux" }
            else if cfg!(target_os = "macos") { "macos" }
            else if cfg!(target_os = "windows") { "windows" }
            else { "unknown" };
            if !self.os.iter().any(|o| o == current_os) {
                return false;
            }
        }
        // Arch check
        if !self.arch.is_empty() {
            let current_arch = if cfg!(target_arch = "x86_64") { "x86_64" }
            else if cfg!(target_arch = "aarch64") { "aarch64" }
            else { "unknown" };
            if !self.arch.iter().any(|a| a == current_arch) {
                return false;
            }
        }
        true
    }

    /// Generate the TOML section for a pandora.toml manifest.
    pub fn to_toml_section(&self) -> String {
        let os_str = self.os.iter().map(|s| format!(""{s}"")).collect::<Vec<_>>().join(", ");
        let arch_str = self.arch.iter().map(|s| format!(""{s}"")).collect::<Vec<_>>().join(", ");
        format!(
            "[compatibility]
pandora = "{}"
os = [{}]
arch = [{}]
permissions = {:?}
sandbox_level = {}
",
            self.pandora_version.as_deref().unwrap_or(">=0.2.0"),
            os_str,
            arch_str,
            self.permissions,
            self.sandbox_level.unwrap_or(0)
        )
    }
}

#[cfg(test)]
mod compat_tests {
    use super::*;

    #[test]
    fn empty_matrix_is_compatible() {
        let m = CompatibilityMatrix::default();
        assert!(m.is_compatible());
    }

    #[test]
    fn current_os_matches() {
        let m = CompatibilityMatrix {
            os: vec!["linux".into()],
            ..Default::default()
        };
        // This test runs on Linux/WSL — should pass
        if cfg!(target_os = "linux") {
            assert!(m.is_compatible());
        }
    }

    #[test]
    fn wrong_os_fails() {
        let m = CompatibilityMatrix {
            os: vec!["nonexistent-os".into()],
            ..Default::default()
        };
        assert!(!m.is_compatible());
    }
}
