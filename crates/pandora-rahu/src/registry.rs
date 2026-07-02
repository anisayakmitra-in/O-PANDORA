use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use pandora_narad::IntentKind;
use thiserror::Error;

use crate::harness::{
    Gene, GeneKind, MetaHarness, MetaHarnessKind, SourceHarness, SourceHarnessKind,
};

#[derive(Debug, Error)]
pub enum RahuError {
    #[error("no source harness registered for intent {0:?}")]
    NoSourceForIntent(IntentKind),

    #[error("no meta harness registered for source {0:?}")]
    NoMetaForSource(SourceHarnessKind),

    #[error("no gene registered for source {0:?} and kind {1:?}")]
    NoGene(SourceHarnessKind, GeneKind),

    #[error("source harness {name:?} of kind {kind:?} is already registered")]
    DuplicateSource {
        kind: SourceHarnessKind,
        name: String,
    },

    #[error("meta harness {name:?} of parent {parent:?} is already registered")]
    DuplicateMeta {
        parent: SourceHarnessKind,
        name: String,
    },

    #[error("gene {name:?} of parent {parent:?} is already registered")]
    DuplicateGene {
        parent: SourceHarnessKind,
        name: String,
    },
}

pub struct SourceHarnessRegistry {
    inner: RwLock<BTreeMap<SourceHarnessKind, Vec<Arc<dyn SourceHarness>>>>,
}

impl SourceHarnessRegistry {
    pub fn new() -> Self {
        SourceHarnessRegistry {
            inner: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn register(&self, harness: Arc<dyn SourceHarness>) -> Result<(), RahuError> {
        let kind = harness.kind();
        let name = harness.name().to_string();
        let mut guard = self.inner.write().expect("registry poisoned");
        let entry = guard.entry(kind).or_default();
        if entry.iter().any(|h| h.name() == name) {
            return Err(RahuError::DuplicateSource { kind, name });
        }
        entry.push(harness);
        Ok(())
    }

    pub fn get(&self, kind: SourceHarnessKind) -> Option<Vec<Arc<dyn SourceHarness>>> {
        let guard = self.inner.read().expect("registry poisoned");
        guard.get(&kind).cloned()
    }

    pub fn first_of(&self, kind: SourceHarnessKind) -> Option<Arc<dyn SourceHarness>> {
        self.get(kind).and_then(|v| v.into_iter().next())
    }

    pub fn resolve_for_intent(&self, intent: IntentKind) -> Result<SourceHarnessKind, RahuError> {
        let guard = self.inner.read().expect("registry poisoned");
        let preferred = match intent {
            IntentKind::Create | IntentKind::Modify | IntentKind::Delete | IntentKind::Execute => {
                SourceHarnessKind::Phoenix
            }
            IntentKind::Read => SourceHarnessKind::Anubis,
            IntentKind::Ask => SourceHarnessKind::Provider,
            IntentKind::Reflect => SourceHarnessKind::Hades,
            IntentKind::Install | IntentKind::Remove => SourceHarnessKind::Shani,
            IntentKind::Verify => SourceHarnessKind::Moira,
            IntentKind::Unknown => SourceHarnessKind::Phoenix,
        };
        if guard.contains_key(&preferred) {
            return Ok(preferred);
        }
        for (kind, list) in guard.iter() {
            if !list.is_empty() {
                return Ok(*kind);
            }
        }
        Err(RahuError::NoSourceForIntent(intent))
    }
}

impl Default for SourceHarnessRegistry {
    fn default() -> Self {
        SourceHarnessRegistry::new()
    }
}

pub struct MetaHarnessRegistry {
    inner: RwLock<BTreeMap<SourceHarnessKind, Vec<Arc<dyn MetaHarness>>>>,
}

impl MetaHarnessRegistry {
    pub fn new() -> Self {
        MetaHarnessRegistry {
            inner: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn register(&self, meta: Arc<dyn MetaHarness>) -> Result<(), RahuError> {
        let parent = meta.parent();
        let name = meta.name().to_string();
        let mut guard = self.inner.write().expect("registry poisoned");
        let entry = guard.entry(parent).or_default();
        if entry.iter().any(|h| h.name() == name) {
            return Err(RahuError::DuplicateMeta { parent, name });
        }
        entry.push(meta);
        Ok(())
    }

    pub fn first_of(&self, parent: SourceHarnessKind) -> Option<Arc<dyn MetaHarness>> {
        let guard = self.inner.read().expect("registry poisoned");
        guard.get(&parent).and_then(|v| v.first().cloned())
    }

    pub fn first_of_kind(
        &self,
        parent: SourceHarnessKind,
        kind: MetaHarnessKind,
    ) -> Option<Arc<dyn MetaHarness>> {
        let guard = self.inner.read().expect("registry poisoned");
        guard
            .get(&parent)
            .and_then(|v| v.iter().find(|m| m.meta_kind() == kind).cloned())
    }
}

impl Default for MetaHarnessRegistry {
    fn default() -> Self {
        MetaHarnessRegistry::new()
    }
}

impl MetaHarnessRegistry {
    /// True if a meta harness with the given name
    /// is registered under the given parent source
    /// harness. The runtime uses this to verify
    /// that a name it expects to find is present;
    /// whether the harness is constitutional is
    /// determined by checking the trait at the
    /// call site.
    pub fn has(&self, parent: SourceHarnessKind, name: &str) -> bool {
        let guard = self.inner.read().expect("registry poisoned");
        guard
            .get(&parent)
            .and_then(|v| v.iter().find(|h| h.name() == name))
            .is_some()
    }
}

pub struct GeneRegistry {
    inner: RwLock<BTreeMap<SourceHarnessKind, Vec<Arc<dyn Gene>>>>,
}

impl GeneRegistry {
    pub fn new() -> Self {
        GeneRegistry {
            inner: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn register(&self, gene: Arc<dyn Gene>) -> Result<(), RahuError> {
        let parent = gene.parent();
        let name = gene.name().to_string();
        let mut guard = self.inner.write().expect("registry poisoned");
        let entry = guard.entry(parent).or_default();
        if entry.iter().any(|g| g.name() == name) {
            return Err(RahuError::DuplicateGene { parent, name });
        }
        entry.push(gene);
        Ok(())
    }

    pub fn first_of(&self, parent: SourceHarnessKind) -> Option<Arc<dyn Gene>> {
        let guard = self.inner.read().expect("registry poisoned");
        guard.get(&parent).and_then(|v| v.first().cloned())
    }

    pub fn first_of_kind(
        &self,
        parent: SourceHarnessKind,
        kind: GeneKind,
    ) -> Option<Arc<dyn Gene>> {
        let guard = self.inner.read().expect("registry poisoned");
        guard
            .get(&parent)
            .and_then(|v| v.iter().find(|g| g.kind() == kind).cloned())
    }
}

impl Default for GeneRegistry {
    fn default() -> Self {
        GeneRegistry::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harness::GeneManifest;
    use pandora_types::constitutional::{ConstitutionalManifest, ManifestKind, ManifestVersion};

    struct DummySource {
        kind: SourceHarnessKind,
        manifest: ConstitutionalManifest,
    }
    impl SourceHarness for DummySource {
        fn kind(&self) -> SourceHarnessKind {
            self.kind
        }
        fn manifest(&self) -> &ConstitutionalManifest {
            &self.manifest
        }
    }

    #[allow(dead_code)]
    struct DummyMeta {
        parent: SourceHarnessKind,
        manifest: ConstitutionalManifest,
    }
    impl MetaHarness for DummyMeta {
        fn meta_kind(&self) -> MetaHarnessKind {
            MetaHarnessKind::General
        }
        fn parent(&self) -> SourceHarnessKind {
            SourceHarnessKind::Phoenix
        }
        fn manifest(&self) -> &ConstitutionalManifest {
            &self.manifest
        }
    }

    struct DummyGene {
        manifest: GeneManifest,
    }
    impl Gene for DummyGene {
        fn manifest(&self) -> &GeneManifest {
            &self.manifest
        }
    }

    #[test]
    fn source_registry_register_and_get() {
        let r = SourceHarnessRegistry::new();
        let h = Arc::new(DummySource {
            kind: SourceHarnessKind::Phoenix,
            manifest: ConstitutionalManifest::new(
                "phoenix",
                ManifestKind::SourceHarness,
                ManifestVersion::new(0, 1, 0),
                "x",
            ),
        });
        r.register(h).unwrap();
        let got = r.first_of(SourceHarnessKind::Phoenix).unwrap();
        assert_eq!(got.name(), "phoenix");
    }

    #[test]
    fn source_registry_duplicate_rejected() {
        let r = SourceHarnessRegistry::new();
        let m1 = ConstitutionalManifest::new(
            "p",
            ManifestKind::SourceHarness,
            ManifestVersion::new(0, 1, 0),
            "x",
        );
        let m2 = ConstitutionalManifest::new(
            "p",
            ManifestKind::SourceHarness,
            ManifestVersion::new(0, 1, 0),
            "x",
        );
        r.register(Arc::new(DummySource {
            kind: SourceHarnessKind::Phoenix,
            manifest: m1,
        }))
        .unwrap();
        let result = r.register(Arc::new(DummySource {
            kind: SourceHarnessKind::Phoenix,
            manifest: m2,
        }));
        assert!(result.is_err());
    }

    #[test]
    fn source_registry_resolve_for_intent() {
        let r = SourceHarnessRegistry::new();
        r.register(Arc::new(DummySource {
            kind: SourceHarnessKind::Phoenix,
            manifest: ConstitutionalManifest::new(
                "p",
                ManifestKind::SourceHarness,
                ManifestVersion::new(0, 1, 0),
                "x",
            ),
        }))
        .unwrap();
        r.register(Arc::new(DummySource {
            kind: SourceHarnessKind::Anubis,
            manifest: ConstitutionalManifest::new(
                "a",
                ManifestKind::SourceHarness,
                ManifestVersion::new(0, 1, 0),
                "x",
            ),
        }))
        .unwrap();
        assert_eq!(
            r.resolve_for_intent(IntentKind::Create).unwrap(),
            SourceHarnessKind::Phoenix
        );
        assert_eq!(
            r.resolve_for_intent(IntentKind::Read).unwrap(),
            SourceHarnessKind::Anubis
        );
    }

    #[test]
    fn meta_registry_register_and_lookup() {
        let r = MetaHarnessRegistry::new();
        r.register(Arc::new(DummyMeta {
            parent: SourceHarnessKind::Phoenix,
            manifest: ConstitutionalManifest::new(
                "phoenix-shell",
                ManifestKind::MetaHarness,
                ManifestVersion::new(0, 1, 0),
                "Phoenix shell meta harness",
            ),
        }))
        .unwrap();
        let m = r
            .first_of_kind(SourceHarnessKind::Phoenix, MetaHarnessKind::General)
            .unwrap();
        assert_eq!(m.name(), "phoenix-shell");
    }

    #[test]
    fn gene_registry_register_and_lookup() {
        let r = GeneRegistry::new();
        r.register(Arc::new(DummyGene {
            manifest: GeneManifest::new(
                SourceHarnessKind::Phoenix,
                GeneKind::Execution,
                "exec-default",
                "0.1.0",
                "Default execution gene",
            ),
        }))
        .unwrap();
        let g = r
            .first_of_kind(SourceHarnessKind::Phoenix, GeneKind::Execution)
            .unwrap();
        assert_eq!(g.name(), "exec-default");
    }
}
