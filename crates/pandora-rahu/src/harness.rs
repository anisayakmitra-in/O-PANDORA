use serde::{Deserialize, Serialize};

use pandora_types::constitutional::{ConstitutionalManifest, ManifestVersion};

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

/// A trait every source harness implements. RAHU
/// interacts with source harnesses only through this
/// trait, so it never hardcodes concrete types.
pub trait SourceHarness: Send + Sync {
    fn manifest(&self) -> &ConstitutionalManifest;
    fn kind(&self) -> SourceHarnessKind;
    fn name(&self) -> &str {
        &self.manifest().identity.name
    }
    fn version(&self) -> ManifestVersion {
        self.manifest().identity.version.clone()
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

pub trait MetaHarness: Send + Sync {
    fn manifest(&self) -> &ConstitutionalManifest;
    fn meta_kind(&self) -> MetaHarnessKind;
    fn name(&self) -> &str {
        &self.manifest().identity.name
    }
    fn parent(&self) -> SourceHarnessKind;
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
    /// Agent - a Gene with identity
    Agent,
    /// SubAgent - nested Agent
    SubAgent,
    /// Swarm - coordinated Agents
    Swarm,
}

impl GeneKind {
    pub fn name(self) -> &'static str {
        match self {
            GeneKind::Read => "read",
            GeneKind::Modify => "modify",
            GeneKind::Execution => "execution",
            GeneKind::Reflection => "reflection",
            GeneKind::Evolution => "evolution",
            GeneKind::Agent => "agent",
            GeneKind::SubAgent => "subagent",
            GeneKind::Swarm => "swarm",
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

    pub fn builder(parent: SourceHarnessKind, kind: GeneKind) -> GeneManifestBuilder {
        GeneManifestBuilder::new(parent, kind)
    }
}

#[derive(Debug)]
pub struct GeneManifestBuilder {
    parent: SourceHarnessKind,
    kind: GeneKind,
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
}

impl GeneManifestBuilder {
    pub fn new(parent: SourceHarnessKind, kind: GeneKind) -> Self {
        Self {
            parent,
            kind,
            name: None,
            version: None,
            description: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> Result<GeneManifest, GeneManifestBuilderError> {
        Ok(GeneManifest {
            parent: self.parent,
            kind: self.kind,
            name: self
                .name
                .ok_or(GeneManifestBuilderError::MissingField("name"))?,
            version: self
                .version
                .ok_or(GeneManifestBuilderError::MissingField("version"))?,
            description: self
                .description
                .ok_or(GeneManifestBuilderError::MissingField("description"))?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GeneManifestBuilderError {
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
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

    #[test]
    fn gene_manifest_builder() {
        let manifest = GeneManifest::builder(SourceHarnessKind::Phoenix, GeneKind::Execution)
            .name("exec-default")
            .version("1.0.0")
            .description("Default execution gene")
            .build()
            .unwrap();
        assert_eq!(manifest.parent, SourceHarnessKind::Phoenix);
        assert_eq!(manifest.kind, GeneKind::Execution);
        assert_eq!(manifest.name, "exec-default");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.description, "Default execution gene");
    }
}
