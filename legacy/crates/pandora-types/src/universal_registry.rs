//! Universal Registry — common trait for all Pandora registries.
//!
//! Every registry (model, provider, gene, harness, skill, workflow, policy,
//! evaluator, package, node, transport, capability) implements this trait.
//! No duplicated registry logic across subsystems.
//!
//! Invariant: "Define a common registry abstraction. Every registry should
//! support discovery, registration, versioning, dependency resolution,
//! capability advertisement, health, provenance, signature verification."

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A registry entry — what every registered item provides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub kind: String,
    pub capabilities: Vec<String>,
    pub health: HealthStatus,
    pub signature: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Health status of a registry entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Degraded,
    Unhealthy,
    Unknown,
}

impl HealthStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Warning => "warning",
            Self::Degraded => "degraded",
            Self::Unhealthy => "unhealthy",
            Self::Unknown => "unknown",
        }
    }
}

/// The universal registry interface.
pub trait Registry {
    /// Register a new entry.
    fn register(&mut self, entry: RegistryEntry) -> Result<(), String>;

    /// Discover entries matching a capability.
    fn discover_by_capability(&self, capability: &str) -> Vec<&RegistryEntry>;

    /// Find an entry by ID.
    fn find(&self, id: &str) -> Option<&RegistryEntry>;

    /// List all entries of a specific kind.
    fn list_by_kind(&self, kind: &str) -> Vec<&RegistryEntry>;

    /// Remove an entry.
    fn unregister(&mut self, id: &str) -> Result<(), String>;

    /// Count total entries.
    fn count(&self) -> usize;
}

/// A simple in-memory registry implementation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InMemoryRegistry {
    entries: HashMap<String, RegistryEntry>,
}

impl InMemoryRegistry {
    pub fn new() -> Self { Self::default() }
}

impl Registry for InMemoryRegistry {
    fn register(&mut self, entry: RegistryEntry) -> Result<(), String> {
        if self.entries.contains_key(&entry.id) {
            return Err(format!("Duplicate entry: {}", entry.id));
        }
        self.entries.insert(entry.id.clone(), entry);
        Ok(())
    }

    fn discover_by_capability(&self, capability: &str) -> Vec<&RegistryEntry> {
        self.entries
            .values()
            .filter(|e| e.capabilities.iter().any(|c| c == capability))
            .collect()
    }

    fn find(&self, id: &str) -> Option<&RegistryEntry> {
        self.entries.get(id)
    }

    fn list_by_kind(&self, kind: &str) -> Vec<&RegistryEntry> {
        self.entries
            .values()
            .filter(|e| e.kind == kind)
            .collect()
    }

    fn unregister(&mut self, id: &str) -> Result<(), String> {
        self.entries
            .remove(id)
            .map(|_| ())
            .ok_or_else(|| format!("Entry not found: {id}"))
    }

    fn count(&self) -> usize { self.entries.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_entry(id: &str, kind: &str) -> RegistryEntry {
        RegistryEntry {
            id: id.into(),
            name: format!("test-{id}"),
            version: "1.0.0".into(),
            kind: kind.into(),
            capabilities: vec!["test".into()],
            health: HealthStatus::Healthy,
            signature: None,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn register_and_find() {
        let mut reg = InMemoryRegistry::new();
        reg.register(test_entry("g1", "gene")).unwrap();
        assert!(reg.find("g1").is_some());
        assert!(reg.find("nope").is_none());
    }

    #[test]
    fn duplicate_rejected() {
        let mut reg = InMemoryRegistry::new();
        reg.register(test_entry("g1", "gene")).unwrap();
        assert!(reg.register(test_entry("g1", "gene")).is_err());
    }

    #[test]
    fn discover_by_capability() {
        let mut reg = InMemoryRegistry::new();
        reg.register(RegistryEntry {
            id: "g1".into(),
            name: "g1".into(),
            version: "1.0.0".into(),
            kind: "gene".into(),
            capabilities: vec!["code".into(), "lint".into()],
            health: HealthStatus::Healthy,
            signature: None,
            metadata: HashMap::new(),
        }).unwrap();
        reg.register(RegistryEntry {
            id: "g2".into(),
            name: "g2".into(),
            version: "1.0.0".into(),
            kind: "gene".into(),
            capabilities: vec!["browser".into()],
            health: HealthStatus::Healthy,
            signature: None,
            metadata: HashMap::new(),
        }).unwrap();
        assert_eq!(reg.discover_by_capability("code").len(), 1);
        assert_eq!(reg.discover_by_capability("browser").len(), 1);
        assert_eq!(reg.discover_by_capability("nonexistent").len(), 0);
    }

    #[test]
    fn list_by_kind() {
        let mut reg = InMemoryRegistry::new();
        reg.register(test_entry("g1", "gene")).unwrap();
        reg.register(test_entry("h1", "harness")).unwrap();
        assert_eq!(reg.list_by_kind("gene").len(), 1);
        assert_eq!(reg.list_by_kind("harness").len(), 1);
        assert_eq!(reg.list_by_kind("skill").len(), 0);
    }

    #[test]
    fn unregister_removes() {
        let mut reg = InMemoryRegistry::new();
        reg.register(test_entry("g1", "gene")).unwrap();
        assert_eq!(reg.count(), 1);
        reg.unregister("g1").unwrap();
        assert_eq!(reg.count(), 0);
    }
}
