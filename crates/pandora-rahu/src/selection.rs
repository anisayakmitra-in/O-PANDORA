use serde::{Deserialize, Serialize};

use crate::harness::{
    Gene, GeneKind, MetaHarness, MetaHarnessKind, SourceHarness, SourceHarnessKind,
};

/// A resolved source harness. RAHU produces this when
/// it has decided which source harness should handle
/// the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceHarnessSelection {
    pub kind: SourceHarnessKind,
    pub name: String,
}

impl SourceHarnessSelection {
    pub fn new(kind: SourceHarnessKind, name: impl Into<String>) -> Self {
        SourceHarnessSelection {
            kind,
            name: name.into(),
        }
    }

    pub fn from_harness(h: &dyn SourceHarness) -> Self {
        SourceHarnessSelection {
            kind: h.kind(),
            name: h.name().to_string(),
        }
    }
}

/// A resolved meta harness within a source harness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaHarnessSelection {
    pub parent: SourceHarnessKind,
    pub name: String,
}

impl MetaHarnessSelection {
    pub fn new(parent: SourceHarnessKind, name: impl Into<String>) -> Self {
        MetaHarnessSelection {
            parent,
            name: name.into(),
        }
    }

    pub fn from_meta(h: &dyn MetaHarness) -> Self {
        MetaHarnessSelection {
            parent: h.parent(),
            name: h.name().to_string(),
        }
    }
}

/// A resolved gene. The gene is the smallest unit of
/// evolution: a runnable action a meta harness can
/// invoke.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneSelection {
    pub parent: SourceHarnessKind,
    pub kind: GeneKind,
    pub name: String,
}

impl GeneSelection {
    pub fn new(parent: SourceHarnessKind, kind: GeneKind, name: impl Into<String>) -> Self {
        GeneSelection {
            parent,
            kind,
            name: name.into(),
        }
    }

    pub fn from_gene(g: &dyn Gene) -> Self {
        GeneSelection {
            parent: g.parent(),
            kind: g.kind(),
            name: g.name().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummySource;
    impl SourceHarness for DummySource {
        fn manifest(&self) -> &crate::harness::SourceHarnessManifest {
            use crate::harness::SourceHarnessManifest;
            use std::sync::OnceLock;
            static M: OnceLock<SourceHarnessManifest> = OnceLock::new();
            M.get_or_init(|| {
                SourceHarnessManifest::new(
                    SourceHarnessKind::Phoenix,
                    "phoenix",
                    "0.1.0",
                    "Execution source harness",
                )
            })
        }
    }

    #[test]
    fn source_selection_from_kind() {
        let s = SourceHarnessSelection::new(SourceHarnessKind::Anubis, "anubis");
        assert_eq!(s.kind, SourceHarnessKind::Anubis);
        assert_eq!(s.name, "anubis");
    }

    #[test]
    fn source_selection_from_harness() {
        let s = SourceHarnessSelection::from_harness(&DummySource);
        assert_eq!(s.kind, SourceHarnessKind::Phoenix);
        assert_eq!(s.name, "phoenix");
    }

    #[test]
    fn meta_selection_construction() {
        let m = MetaHarnessSelection::new(SourceHarnessKind::Phoenix, "phoenix-shell");
        assert_eq!(m.parent, SourceHarnessKind::Phoenix);
        assert_eq!(m.name, "phoenix-shell");
    }

    #[test]
    fn gene_selection_construction() {
        let g = GeneSelection::new(
            SourceHarnessKind::Phoenix,
            GeneKind::Execution,
            "exec-default",
        );
        assert_eq!(g.parent, SourceHarnessKind::Phoenix);
        assert_eq!(g.kind, GeneKind::Execution);
        assert_eq!(g.name, "exec-default");
    }
}
