use serde::{Deserialize, Serialize};

/// The kind of source harness. Each source harness
/// owns its own meta harnesses and genes. RAHU only
/// knows the *kind*, not the implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SourceHarnessKind {
    /// Execution source harness (Phoenix).
    Phoenix,
    /// Memory source harness (ANUBIS).
    Anubis,
    /// Decision source harness (MOIRA).
    Moira,
    /// Soul / identity source harness (HADES).
    Hades,
    /// Evolution source harness (SHANI).
    Shani,
    /// Provider source harness (model providers).
    Provider,
}

impl SourceHarnessKind {
    pub fn name(self) -> &'static str {
        match self {
            SourceHarnessKind::Phoenix => "Phoenix",
            SourceHarnessKind::Anubis => "ANUBIS",
            SourceHarnessKind::Moira => "MOIRA",
            SourceHarnessKind::Hades => "HADES",
            SourceHarnessKind::Shani => "SHANI",
            SourceHarnessKind::Provider => "Provider",
        }
    }

    pub fn all() -> &'static [SourceHarnessKind] {
        &[
            SourceHarnessKind::Phoenix,
            SourceHarnessKind::Anubis,
            SourceHarnessKind::Moira,
            SourceHarnessKind::Hades,
            SourceHarnessKind::Shani,
            SourceHarnessKind::Provider,
        ]
    }
}

/// A source-harness manifest. Describes a registered
/// source harness without revealing its implementation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceHarnessManifest {
    pub kind: SourceHarnessKind,
    pub name: String,
    pub version: String,
    pub description: String,
}

impl SourceHarnessManifest {
    pub fn new(
        kind: SourceHarnessKind,
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        SourceHarnessManifest {
            kind,
            name: name.into(),
            version: version.into(),
            description: description.into(),
        }
    }
}

/// A trait every source harness implements. RAHU
/// interacts with source harnesses only through this
/// trait, so it never hardcodes concrete types.
pub trait SourceHarness: Send + Sync {
    fn manifest(&self) -> &SourceHarnessManifest;
    fn kind(&self) -> SourceHarnessKind {
        self.manifest().kind
    }
    fn name(&self) -> &str {
        &self.manifest().name
    }
}

/// The kind of meta harness. Each meta harness belongs
/// to a single source harness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MetaHarnessKind {
    /// General meta harness.
    General,
    /// Shell / command execution meta harness.
    Shell,
    /// Filesystem meta harness.
    Filesystem,
    /// Provider invocation meta harness.
    Provider,
    /// Memory access meta harness.
    Memory,
    /// Network meta harness.
    Network,
}

impl MetaHarnessKind {
    pub fn name(self) -> &'static str {
        match self {
            MetaHarnessKind::General => "general",
            MetaHarnessKind::Shell => "shell",
            MetaHarnessKind::Filesystem => "filesystem",
            MetaHarnessKind::Provider => "provider",
            MetaHarnessKind::Memory => "memory",
            MetaHarnessKind::Network => "network",
        }
    }
}

/// A meta-harness manifest. Meta harnesses are
/// subsystems of a source harness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaHarnessManifest {
    pub parent: SourceHarnessKind,
    pub kind: MetaHarnessKind,
    pub name: String,
    pub version: String,
}

impl MetaHarnessManifest {
    pub fn new(
        parent: SourceHarnessKind,
        kind: MetaHarnessKind,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        MetaHarnessManifest {
            parent,
            kind,
            name: name.into(),
            version: version.into(),
        }
    }
}

pub trait MetaHarness: Send + Sync {
    fn manifest(&self) -> &MetaHarnessManifest;
    fn name(&self) -> &str {
        &self.manifest().name
    }
    fn parent(&self) -> SourceHarnessKind {
        self.manifest().parent
    }
}

/// The kind of gene. Genes are the smallest unit of
/// evolution: a runnable action a meta harness can
/// invoke.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GeneKind {
    /// Read-only / inspection.
    Read,
    /// Mutation / modification.
    Modify,
    /// Pure execution.
    Execution,
    /// Reflection / synthesis.
    Reflection,
    /// Evolution / mutation generation.
    Evolution,
}

impl GeneKind {
    pub fn name(self) -> &'static str {
        match self {
            GeneKind::Read => "read",
            GeneKind::Modify => "modify",
            GeneKind::Execution => "execution",
            GeneKind::Reflection => "reflection",
            GeneKind::Evolution => "evolution",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneManifest {
    pub parent: SourceHarnessKind,
    pub kind: GeneKind,
    pub name: String,
    pub version: String,
    pub description: String,
}

impl GeneManifest {
    pub fn new(
        parent: SourceHarnessKind,
        kind: GeneKind,
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        GeneManifest {
            parent,
            kind,
            name: name.into(),
            version: version.into(),
            description: description.into(),
        }
    }
}

pub trait Gene: Send + Sync {
    fn manifest(&self) -> &GeneManifest;
    fn name(&self) -> &str {
        &self.manifest().name
    }
    fn parent(&self) -> SourceHarnessKind {
        self.manifest().parent
    }
    fn kind(&self) -> GeneKind {
        self.manifest().kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_harness_kind_names() {
        assert_eq!(SourceHarnessKind::Phoenix.name(), "Phoenix");
        assert_eq!(SourceHarnessKind::Anubis.name(), "ANUBIS");
        assert_eq!(SourceHarnessKind::Shani.name(), "SHANI");
    }

    #[test]
    fn source_harness_kind_all_is_complete() {
        for kind in SourceHarnessKind::all() {
            assert!(!kind.name().is_empty());
        }
    }

    #[test]
    fn meta_harness_kind_names() {
        assert_eq!(MetaHarnessKind::Shell.name(), "shell");
        assert_eq!(MetaHarnessKind::Memory.name(), "memory");
    }

    #[test]
    fn gene_kind_names() {
        assert_eq!(GeneKind::Execution.name(), "execution");
        assert_eq!(GeneKind::Reflection.name(), "reflection");
    }
}
