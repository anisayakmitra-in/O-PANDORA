use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::error::{HarnessError, Result};
use crate::manifest::HarnessManifest;
use crate::roles::HarnessRole;
use crate::traits::Harness;

/// Registry for managing harnesses.
///
/// Provides registration, lookup, and role-based filtering of harnesses.
/// Thread-safe for concurrent access.
#[derive(Default)]
pub struct Registry {
    harnesses: RwLock<HashMap<String, Arc<dyn Harness + Send + Sync>>>,
    manifests: RwLock<HashMap<String, HarnessManifest>>,
}

impl Registry {
    /// Creates a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a harness with its manifest.
    ///
    /// Returns an error if a harness with the same ID already exists.
    pub fn register(
        &self,
        harness: Arc<dyn Harness + Send + Sync>,
        manifest: HarnessManifest,
    ) -> Result<()> {
        let id = manifest.id.clone();

        // Check for duplicate
        if self.manifests.read().unwrap().contains_key(&id) {
            return Err(HarnessError::AlreadyRegistered(id));
        }

        // Store manifest first
        self.manifests.write().unwrap().insert(id.clone(), manifest);

        // Store harness
        self.harnesses.write().unwrap().insert(id, harness);

        Ok(())
    }

    /// Unregisters a harness by ID.
    pub fn unregister(&self, id: &str) -> Result<()> {
        self.manifests.write().unwrap().remove(id);
        self.harnesses.write().unwrap().remove(id);
        Ok(())
    }

    /// Gets a harness by ID.
    pub fn get(&self, id: &str) -> Option<Arc<dyn Harness + Send + Sync>> {
        self.harnesses.read().unwrap().get(id).cloned()
    }

    /// Gets a manifest by ID.
    pub fn get_manifest(&self, id: &str) -> Option<HarnessManifest> {
        self.manifests.read().unwrap().get(id).cloned()
    }

    /// Returns all registered harness IDs.
    pub fn list_ids(&self) -> Vec<String> {
        self.manifests.read().unwrap().keys().cloned().collect()
    }

    /// Returns all registered manifests.
    pub fn list_manifests(&self) -> Vec<HarnessManifest> {
        self.manifests.read().unwrap().values().cloned().collect()
    }

    /// Filters harnesses by role.
    pub fn filter_by_role(&self, role: HarnessRole) -> Vec<HarnessManifest> {
        self.manifests
            .read()
            .unwrap()
            .values()
            .filter(|m| m.role == role)
            .cloned()
            .collect()
    }

    /// Returns the number of registered harnesses.
    pub fn len(&self) -> usize {
        self.manifests.read().unwrap().len()
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.manifests.read().unwrap().is_empty()
    }

    /// Checks if a harness is registered.
    pub fn contains(&self, id: &str) -> bool {
        self.manifests.read().unwrap().contains_key(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::roles::HarnessRole;
    use crate::traits::Harness;

    #[derive(Debug)]
    struct TestHarness {
        id: String,
        role: HarnessRole,
    }

    impl Harness for TestHarness {
        fn id(&self) -> &str {
            &self.id
        }

        fn name(&self) -> &str {
            "Test"
        }

        fn version(&self) -> &str {
            "0.1.0"
        }

        fn role(&self) -> HarnessRole {
            self.role
        }

        fn initialize(&mut self) -> std::result::Result<(), String> {
            Ok(())
        }

        fn shutdown(&mut self) -> std::result::Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_registry_register_and_get() {
        let registry = Registry::new();
        let harness = Arc::new(TestHarness {
            id: "test-1".into(),
            role: HarnessRole::Planning,
        });
        let manifest = HarnessManifest {
            id: "test-1".into(),
            name: "Test".into(),
            version: "0.1.0".into(),
            author: "test".into(),
            role: HarnessRole::Planning,
            description: "Test harness".into(),
            dependencies: vec![],
        };

        registry.register(harness.clone(), manifest).unwrap();
        assert!(registry.contains("test-1"));
        assert_eq!(registry.len(), 1);
        assert_eq!(registry.get("test-1").unwrap().id(), "test-1");
    }

    #[test]
    fn test_registry_duplicate_rejected() {
        let registry = Registry::new();
        let harness = Arc::new(TestHarness {
            id: "test-1".into(),
            role: HarnessRole::Planning,
        });
        let manifest = HarnessManifest {
            id: "test-1".into(),
            name: "Test".into(),
            version: "0.1.0".into(),
            author: "test".into(),
            role: HarnessRole::Planning,
            description: "Test harness".into(),
            dependencies: vec![],
        };

        registry
            .register(harness.clone(), manifest.clone())
            .unwrap();
        let result = registry.register(harness, manifest);
        assert!(matches!(result, Err(HarnessError::AlreadyRegistered(_))));
    }

    #[test]
    fn test_registry_filter_by_role() {
        let registry = Registry::new();

        let h1 = Arc::new(TestHarness {
            id: "h1".into(),
            role: HarnessRole::Planning,
        });
        let m1 = HarnessManifest {
            id: "h1".into(),
            name: "H1".into(),
            version: "0.1.0".into(),
            author: "a".into(),
            role: HarnessRole::Planning,
            description: "".into(),
            dependencies: vec![],
        };
        registry.register(h1, m1).unwrap();

        let h2 = Arc::new(TestHarness {
            id: "h2".into(),
            role: HarnessRole::Validation,
        });
        let m2 = HarnessManifest {
            id: "h2".into(),
            name: "H2".into(),
            version: "0.1.0".into(),
            author: "a".into(),
            role: HarnessRole::Validation,
            description: "".into(),
            dependencies: vec![],
        };
        registry.register(h2, m2).unwrap();

        let planning = registry.filter_by_role(HarnessRole::Planning);
        assert_eq!(planning.len(), 1);
        assert_eq!(planning[0].id, "h1");
    }
}
