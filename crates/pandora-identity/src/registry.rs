use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use thiserror::Error;

use crate::kind::IdentityKind;
use crate::manifest::Identity;

/// An entry in the . The registry
/// stores the identity along with the trait object so
/// the runtime can dispatch through the  trait.
#[derive(Debug, Clone)]
pub struct IdentityEntry {
    /// The identity (cloned manifest + Arc to the trait
    /// object).
    pub identity: Arc<dyn Identity>,
}

impl IdentityEntry {
    pub fn new(identity: Arc<dyn Identity>) -> Self {
        IdentityEntry { identity }
    }

    pub fn id(&self) -> &str {
        &self.identity.manifest().id
    }

    pub fn kind(&self) -> IdentityKind {
        self.identity.manifest().kind
    }
}

/// Errors the identity registry can produce.
#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("identity {0:?} is already registered")]
    Duplicate(String),

    #[error("identity {0:?} is not registered")]
    NotFound(String),

    #[error("identity {id} has the wrong kind: expected {expected:?}, got {got:?}")]
    WrongKind {
        id: String,
        expected: IdentityKind,
        got: IdentityKind,
    },
}

/// A registry of constitutional identities. The
/// registry is keyed by identity id and provides
/// lookup by id, by kind, and by (kind, id) pair.
pub struct IdentityRegistry {
    inner: RwLock<BTreeMap<String, IdentityEntry>>,
}

impl IdentityRegistry {
    pub fn new() -> Self {
        IdentityRegistry {
            inner: RwLock::new(BTreeMap::new()),
        }
    }

    /// Register an identity. The id must be unique.
    pub fn register(&self, identity: Arc<dyn Identity>) -> Result<(), IdentityError> {
        let id = identity.manifest().id.clone();
        let mut guard = self.inner.write().expect("registry poisoned");
        if guard.contains_key(&id) {
            return Err(IdentityError::Duplicate(id));
        }
        guard.insert(id, IdentityEntry::new(identity));
        Ok(())
    }

    /// Get an identity by id.
    pub fn get(&self, id: &str) -> Option<IdentityEntry> {
        let guard = self.inner.read().expect("registry poisoned");
        guard.get(id).cloned()
    }

    /// Get all identities of a given kind.
    pub fn of_kind(&self, kind: IdentityKind) -> Vec<IdentityEntry> {
        let guard = self.inner.read().expect("registry poisoned");
        guard
            .values()
            .filter(|e| e.kind() == kind)
            .cloned()
            .collect()
    }

    /// Number of registered identities.
    pub fn len(&self) -> usize {
        let guard = self.inner.read().expect("registry poisoned");
        guard.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over all identities.
    pub fn iter(&self) -> Vec<IdentityEntry> {
        let guard = self.inner.read().expect("registry poisoned");
        guard.values().cloned().collect()
    }

    /// Verify a (kind, id) pair is registered and the
    /// stored identity has the expected kind. Returns
    /// the entry on success.
    pub fn verify(&self, id: &str, expected: IdentityKind) -> Result<IdentityEntry, IdentityError> {
        let entry = self
            .get(id)
            .ok_or_else(|| IdentityError::NotFound(id.to_string()))?;
        if entry.kind() != expected {
            return Err(IdentityError::WrongKind {
                id: id.to_string(),
                expected,
                got: entry.kind(),
            });
        }
        Ok(entry)
    }
}

impl Default for IdentityRegistry {
    fn default() -> Self {
        IdentityRegistry::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{Identity, IdentityManifest};

    #[derive(Debug)]
    struct StubIdentity {
        manifest: IdentityManifest,
    }
    impl Identity for StubIdentity {
        fn manifest(&self) -> &IdentityManifest {
            &self.manifest
        }
    }

    fn stub(id: &str, kind: IdentityKind) -> Arc<dyn Identity> {
        Arc::new(StubIdentity {
            manifest: IdentityManifest::new(id, id, kind, "kuber"),
        })
    }

    #[test]
    fn register_and_get() {
        let r = IdentityRegistry::new();
        r.register(stub("a", IdentityKind::SourceHarness)).unwrap();
        let got = r.get("a").unwrap();
        assert_eq!(got.id(), "a");
    }

    #[test]
    fn duplicate_id_rejected() {
        let r = IdentityRegistry::new();
        r.register(stub("a", IdentityKind::SourceHarness)).unwrap();
        let result = r.register(stub("a", IdentityKind::SourceHarness));
        assert!(matches!(result, Err(IdentityError::Duplicate(_))));
    }

    #[test]
    fn of_kind_filters() {
        let r = IdentityRegistry::new();
        r.register(stub("s1", IdentityKind::SourceHarness)).unwrap();
        r.register(stub("s2", IdentityKind::SourceHarness)).unwrap();
        r.register(stub("m1", IdentityKind::MetaHarness)).unwrap();
        let sources = r.of_kind(IdentityKind::SourceHarness);
        assert_eq!(sources.len(), 2);
        let metas = r.of_kind(IdentityKind::MetaHarness);
        assert_eq!(metas.len(), 1);
    }

    #[test]
    fn verify_kind_mismatch() {
        let r = IdentityRegistry::new();
        r.register(stub("x", IdentityKind::SourceHarness)).unwrap();
        let result = r.verify("x", IdentityKind::MetaHarness);
        assert!(matches!(result, Err(IdentityError::WrongKind { .. })));
    }

    #[test]
    fn verify_success() {
        let r = IdentityRegistry::new();
        r.register(stub("x", IdentityKind::SourceHarness)).unwrap();
        let result = r.verify("x", IdentityKind::SourceHarness);
        assert!(result.is_ok());
    }

    #[test]
    fn empty_registry() {
        let r = IdentityRegistry::new();
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
    }
}
