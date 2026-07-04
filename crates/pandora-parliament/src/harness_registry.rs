//! Harness Registry — the Shadow Council's harness lifecycle manager.
//!
//! Manages Source, Meta, and Domain Harnesses with a unified lifecycle:
//!   register, unregister, enable, disable, health, list_by_kind
//!
//! All harness types share the same `Harness` trait. The registry
//! stores `Box<dyn Harness>` and discriminates by `HarnessKind`.

use pandora_types::harness::{Harness, HarnessKind, HarnessManifest};
use std::collections::HashMap;

/// Runtime state of a registered harness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessState {
    Registered,
    Enabled,
    Disabled,
    Error(String),
}

/// A registered harness with its runtime state.
#[derive(Debug)]
pub struct HarnessEntry {
    pub manifest: HarnessManifest,
    pub state: HarnessState,
}

/// The Shadow Council's harness registry.
#[derive(Debug)]
pub struct HarnessRegistry {
    harnesses: HashMap<String, Box<dyn Harness>>,
    states: HashMap<String, HarnessState>,
}

impl HarnessRegistry {
    pub fn new() -> Self {
        Self {
            harnesses: HashMap::new(),
            states: HashMap::new(),
        }
    }

    /// Register a harness. Stores it in Registered state.
    pub fn register(&mut self, harness: Box<dyn Harness>) -> Result<(), String> {
        let id = harness.id().to_string();
        if self.harnesses.contains_key(&id) {
            return Err(format!("Harness already registered: {}", id));
        }
        self.states.insert(id.clone(), HarnessState::Registered);
        self.harnesses.insert(id, harness);
        Ok(())
    }

    /// Unregister and shutdown a harness.
    pub fn unregister(&mut self, id: &str) -> Result<(), String> {
        let mut h = self.harnesses.remove(id).ok_or(format!("Harness not found: {}", id))?;
        h.shutdown().ok();
        self.states.remove(id);
        Ok(())
    }

    /// Enable — calls initialize() and transitions to Enabled.
    pub fn enable(&mut self, id: &str) -> Result<(), String> {
        let h = self.harnesses.get_mut(id).ok_or(format!("Harness not found: {}", id))?;
        h.initialize()?;
        self.states.insert(id.to_string(), HarnessState::Enabled);
        Ok(())
    }

    /// Disable — calls shutdown() and transitions to Disabled.
    pub fn disable(&mut self, id: &str) -> Result<(), String> {
        if let Some(h) = self.harnesses.get_mut(id) {
            h.shutdown().ok();
        }
        self.states.insert(id.to_string(), HarnessState::Disabled);
        Ok(())
    }

    /// Health check.
    pub fn health(&self, id: &str) -> Result<(), String> {
        if let Some(h) = self.harnesses.get(id) {
            h.health()
        } else {
            Err(format!("Harness not found: {}", id))
        }
    }

    /// List all harnesses of a given kind.
    pub fn list_by_kind(&self, kind: &HarnessKind) -> Vec<&dyn Harness> {
        self.harnesses
            .values()
            .filter(|h| h.kind() == kind)
            .map(|h| h.as_ref())
            .collect()
    }

    /// List all registered harnesses with their state.
    pub fn all_entries(&self) -> Vec<(&dyn Harness, &HarnessState)> {
        self.harnesses
            .values()
            .map(|h| {
                let state = self.states.get(h.id()).unwrap_or(&HarnessState::Registered);
                (h.as_ref(), state)
            })
            .collect()
    }

    /// Get a single harness by ID.
    pub fn get(&self, id: &str) -> Option<&dyn Harness> {
        self.harnesses.get(id).map(|h| h.as_ref())
    }

    /// Count by kind.
    pub fn count_by_kind(&self, kind: &HarnessKind) -> usize {
        self.harnesses.values().filter(|h| h.kind() == kind).count()
    }

    pub fn total_count(&self) -> usize {
        self.harnesses.len()
    }
}

impl Default for HarnessRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_types::harness::HarnessManifestBuilder;

    /// Minimal test harness for testing the registry.
    #[derive(Debug)]
    struct TestHarness {
        manifest: HarnessManifest,
    }

    impl Harness for TestHarness {
        fn manifest(&self) -> &HarnessManifest { &self.manifest }
    }

    fn make_test_harness(id: &str, kind: HarnessKind) -> TestHarness {
        TestHarness {
            manifest: HarnessManifestBuilder::default()
                .id(id)
                .name(id)
                .version("0.1.0")
                .author("test")
                .kind(kind)
                .build()
                .unwrap(),
        }
    }

    #[test]
    fn register_and_list() {
        let mut reg = HarnessRegistry::new();
        reg.register(Box::new(make_test_harness("anubis", HarnessKind::Source))).unwrap();
        reg.register(Box::new(make_test_harness("shani", HarnessKind::Meta))).unwrap();
        reg.register(Box::new(make_test_harness("research", HarnessKind::Domain))).unwrap();

        assert_eq!(reg.total_count(), 3);
        assert_eq!(reg.count_by_kind(&HarnessKind::Source), 1);
        assert_eq!(reg.count_by_kind(&HarnessKind::Meta), 1);
        assert_eq!(reg.count_by_kind(&HarnessKind::Domain), 1);
    }

    #[test]
    fn enable_disable_cycle() {
        let mut reg = HarnessRegistry::new();
        reg.register(Box::new(make_test_harness("test", HarnessKind::Source))).unwrap();
        reg.enable("test").unwrap();
        let entry = reg.all_entries();
        assert_eq!(entry[0].1, &HarnessState::Enabled);
        reg.disable("test").unwrap();
        let entry = reg.all_entries();
        assert_eq!(entry[0].1, &HarnessState::Disabled);
    }

    #[test]
    fn unregister_removes() {
        let mut reg = HarnessRegistry::new();
        reg.register(Box::new(make_test_harness("x", HarnessKind::Meta))).unwrap();
        assert_eq!(reg.total_count(), 1);
        reg.unregister("x").unwrap();
        assert_eq!(reg.total_count(), 0);
    }

    #[test]
    fn duplicate_register_rejected() {
        let mut reg = HarnessRegistry::new();
        reg.register(Box::new(make_test_harness("dup", HarnessKind::Source))).unwrap();
        assert!(reg.register(Box::new(make_test_harness("dup", HarnessKind::Source))).is_err());
    }

    #[test]
    fn missing_harness_returns_error() {
        let reg = HarnessRegistry::new();
        assert!(reg.get("nonexistent").is_none());
        assert!(reg.health("nonexistent").is_err());
    }
}
