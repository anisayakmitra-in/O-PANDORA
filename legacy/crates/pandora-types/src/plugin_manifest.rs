//! Plugin Manifest System — unified manifest format for all Pandora components.
//!
//! Every component type (harness, gene, workflow, policy, evaluator, skill,
//! provider, connector, transport, sandbox, runtime node) uses the same manifest
//! structure. Adding a new component type = new manifest file, not new code.
//!
//! Invariant: "Everything should be installable via manifest."

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The kind of component this manifest describes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginKind {
    Harness,
    Gene,
    Workflow,
    Policy,
    Evaluator,
    Skill,
    Provider,
    Connector,
    Transport,
    Sandbox,
    RuntimeNode,
    Custom(String),
}

impl PluginKind {
    pub fn label(&self) -> &str {
        match self {
            Self::Harness => "harness",
            Self::Gene => "gene",
            Self::Workflow => "workflow",
            Self::Policy => "policy",
            Self::Evaluator => "evaluator",
            Self::Skill => "skill",
            Self::Provider => "provider",
            Self::Connector => "connector",
            Self::Transport => "transport",
            Self::Sandbox => "sandbox",
            Self::RuntimeNode => "runtime-node",
            Self::Custom(s) => s,
        }
    }
}

/// A dependency on another plugin/package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub name: String,
    pub version: String,
    pub required: bool,
}

/// A capability this plugin advertises.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCapability {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub confidence: f32,
}

/// The unified plugin manifest — one format for every component type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub kind: PluginKind,
    pub description: String,
    pub author: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub entrypoint: Option<String>,
    pub dependencies: Vec<PluginDependency>,
    pub capabilities: Vec<PluginCapability>,
    pub permissions: Option<serde_json::Value>, // PermissionManifest JSON
    pub hooks: Option<serde_json::Value>,       // HookManifest JSON
    pub metadata: HashMap<String, String>,
}

impl PluginManifest {
    /// Create a minimal manifest.
    pub fn new(name: &str, version: &str, kind: PluginKind, description: &str) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            kind,
            description: description.into(),
            author: None,
            license: Some("MIT".into()),
            repository: None,
            entrypoint: None,
            dependencies: vec![],
            capabilities: vec![],
            permissions: None,
            hooks: None,
            metadata: HashMap::new(),
        }
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| format!("serialize: {e}"))
    }

    /// Deserialize from JSON.
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("deserialize: {e}"))
    }

    /// Check if all required dependencies are present in a list of available plugins.
    pub fn check_dependencies(&self, available: &[&str]) -> Result<(), Vec<String>> {
        let mut missing = vec![];
        for dep in &self.dependencies {
            if dep.required && !available.contains(&dep.name.as_str()) {
                missing.push(dep.name.clone());
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_json() {
        let m = PluginManifest::new(
            "coding-domain",
            "1.0.0",
            PluginKind::Harness,
            "Coding domain harness",
        );
        let json = m.to_json().unwrap();
        let parsed = PluginManifest::from_json(&json).unwrap();
        assert_eq!(parsed.name, "coding-domain");
        assert_eq!(parsed.kind, PluginKind::Harness);
    }

    #[test]
    fn dependency_check() {
        let mut m = PluginManifest::new("test-plugin", "1.0.0", PluginKind::Skill, "test");
        m.dependencies.push(PluginDependency {
            name: "rust-analyzer".into(),
            version: ">=1.0".into(),
            required: true,
        });
        assert!(m.check_dependencies(&["rust-analyzer", "git"]).is_ok());
        assert!(m.check_dependencies(&["git"]).is_err());
    }

    #[test]
    fn optionals_are_none() {
        let m = PluginManifest::new("test", "0.1.0", PluginKind::Gene, "test gene");
        assert!(m.author.is_none());
        assert!(m.permissions.is_none());
        assert!(m.hooks.is_none());
    }
}
