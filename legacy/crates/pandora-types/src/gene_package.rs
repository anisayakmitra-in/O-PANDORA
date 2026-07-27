//! Gene Package — on-disk gene format.
//!
//! A Gene Package is a directory with:
//!   `<name>`/
//!   ├── gene.toml    # manifest
//!   ├── src/
//!   │   └── lib.rs   # implementation
//!   └── README.md    # optional

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A discovered gene on disk.
#[derive(Debug, Clone)]
pub struct GenePackage {
    pub root: PathBuf,
    pub manifest: GenePackageManifest,
}

/// The gene.toml manifest — mirrors GeneManifest for filesystem discovery.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenePackageManifest {
    pub id: String,
    pub name: String,
    pub kind: String, // matches GeneKind::as_str()
    pub version: String,
    pub author: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub slash_commands: Vec<SlashCommandDef>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SlashCommandDef {
    pub command: String,
    pub description: String,
}

/// Scans a directory for gene packages (directories containing gene.toml).

/// Standard packages directory: $PANDORA_HOME/packages/ or ~/.pandora/packages/
pub fn packages_dir() -> std::path::PathBuf {
    let base = std::env::var("PANDORA_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            std::path::PathBuf::from(home).join(".pandora")
        });
    base.join("packages")
}

pub fn discover_gene_packages(root: &str) -> Vec<GenePackage> {
    let mut packages = Vec::new();
    let dir = match std::fs::read_dir(root) {
        Ok(d) => d,
        Err(_) => return packages,
    };
    for entry in dir.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let toml_path = path.join("gene.toml");
        if !toml_path.exists() {
            continue;
        }
        let content = match std::fs::read_to_string(&toml_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let manifest: GenePackageManifest = match toml::from_str(&content) {
            Ok(m) => m,
            Err(_) => continue,
        };
        packages.push(GenePackage {
            root: path,
            manifest,
        });
    }
    packages
}
