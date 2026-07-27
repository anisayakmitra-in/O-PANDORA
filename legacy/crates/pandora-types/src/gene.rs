//! Canonical Gene types — the universal building block of Pandora.
//!
//! Genes are small composable runtime units. Every capability is a Gene.
//! They can exist alone or inside a Harness. The `Gene` trait is the
//! primary extension API for Pandora.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Gene Category ──

/// Top-level classification of a gene's domain.
/// Every GeneKind belongs to exactly one category.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GeneCategory {
    /// Execution: workflow, planner, executor, scheduler, automation
    Execution,
    /// Memory: retrieval, embedding, compression, archive, cache
    Memory,
    /// Infrastructure: provider, connector, bridge, gateway, registry
    Infrastructure,
    /// Reasoning: reasoning, reflection, critic, judge, evaluation
    Reasoning,
    /// Security: policy, verification, sandbox, audit, redteam
    Security,
    /// Networking: communication, network, federation, replication, synchronization
    Networking,
    /// Multimodal: vision, voice, robotics, browser, shell
    Multimodal,
    /// Research: simulation, world_model, cognitive_model, benchmark, optimization
    Research,
}

impl GeneCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Execution => "execution",
            Self::Memory => "memory",
            Self::Infrastructure => "infrastructure",
            Self::Reasoning => "reasoning",
            Self::Security => "security",
            Self::Networking => "networking",
            Self::Multimodal => "multimodal",
            Self::Research => "research",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Execution => "Execution",
            Self::Memory => "Memory",
            Self::Infrastructure => "Infrastructure",
            Self::Reasoning => "Reasoning",
            Self::Security => "Security",
            Self::Networking => "Networking",
            Self::Multimodal => "Multimodal",
            Self::Research => "Research",
        }
    }
}

// ── Gene Kind ──

/// Classification of a gene's purpose.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum GeneKind {
    // ── Tool genes — atomic capabilities ──
    Tool,
    Provider,
    Workflow,
    Skill,
    SlashCommand,
    MCP,
    // ── Runtime genes — execution and coordination ──
    Agent,
    Memory,
    Planner,
    Reasoner,
    Execution,
    // ── Governance genes — constitutional control ──
    Governance,
    Security,
    Permission,
    // ── Infrastructure genes — deploy, network, storage ──
    Infrastructure,
    Communication,
    // ── Evolution genes — self-improvement ──
    Evolution,
    Cognitive,
    // ── Evaluation genes ──
    Knowledge,
    Benchmark,
    /// User-defined gene kind.
    Custom(String),
}

impl GeneKind {
    /// Canonical string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tool => "tool",
            Self::Provider => "provider",
            Self::Workflow => "workflow",
            Self::Skill => "skill",
            Self::SlashCommand => "slash_command",
            Self::MCP => "mcp",
            Self::Agent => "agent",
            Self::Memory => "memory",
            Self::Planner => "planner",
            Self::Reasoner => "reasoner",
            Self::Execution => "execution",
            Self::Governance => "governance",
            Self::Security => "security",
            Self::Permission => "permission",
            Self::Infrastructure => "infrastructure",
            Self::Communication => "communication",
            Self::Evolution => "evolution",
            Self::Cognitive => "cognitive",
            Self::Knowledge => "knowledge",
            Self::Benchmark => "benchmark",
            Self::Custom(_) => "custom",
        }
    }

    /// Which category this gene kind belongs to.
    pub fn category(&self) -> GeneCategory {
        match self {
            Self::Tool | Self::Workflow | Self::SlashCommand => GeneCategory::Execution,
            Self::Planner | Self::Execution | Self::Agent => GeneCategory::Execution,
            Self::Memory | Self::Knowledge => GeneCategory::Memory,
            Self::Provider | Self::Infrastructure | Self::Communication | Self::MCP => {
                GeneCategory::Infrastructure
            }
            Self::Reasoner | Self::Cognitive | Self::Benchmark => GeneCategory::Reasoning,
            Self::Governance | Self::Security | Self::Permission => GeneCategory::Security,
            Self::Evolution => GeneCategory::Research,
            Self::Skill => GeneCategory::Execution,
            Self::Custom(_) => GeneCategory::Execution,
        }
    }
}

// ── Gene Metadata ──

/// Rich metadata for a Gene — used by KUBER for display, search, and publishing.
/// Not part of the runtime execution path.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneMetadata {
    pub description: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub tags: Vec<String>,
    pub documentation: Option<String>,
    pub icon: Option<String>,
    pub examples: Vec<String>,
    pub custom: HashMap<String, String>,
    pub permissions: Vec<String>,
    // ── K-O Palace publishing metadata ──
    /// Trust level: Experimental, Community, Verified, Official, Enterprise, Certified
    pub trust_level: String,
    /// Capabilities provided by this gene (e.g. ["filesystem.read", "filesystem.write"])
    pub capabilities_provided: Vec<String>,
    /// Capabilities required from other genes (dependency on capabilities, not names)
    pub capabilities_required: Vec<String>,
    /// Minimum Pandora version compatibility
    pub min_pandora_version: Option<String>,
    /// SHA256 hash of the published artifact
    pub content_hash: Option<String>,
    /// Ed25519 signature of the content hash
    pub signature: Option<String>,
    /// Publisher identity
    pub publisher: Option<String>,
    /// Download count from K-O Palace
    pub downloads: u64,
    /// Success rate (0.0-1.0) from K-O Palace telemetry
    pub success_rate: f64,
}

// ── Gene Manifest — minimalist runtime record ──

/// Canonical Gene manifest — minimal runtime fields only.
/// Rich metadata lives in `GeneMetadata`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneManifest {
    pub id: String,
    pub name: String,
    pub kind: GeneKind,
    pub version: String,
    pub author: String,
    pub dependencies: Vec<String>,
    pub capabilities: Vec<String>,
    pub slash_commands: Vec<super::harness::SlashCommand>,
    pub owner_harness: Option<String>,
    pub metadata: GeneMetadata,
}

impl GeneManifest {
    /// Create a new builder for constructing a manifest.
    pub fn builder() -> GeneManifestBuilder {
        GeneManifestBuilder::default()
    }
}

// ── Gene Manifest Builder ──

/// Builder for constructing a `GeneManifest` with validation.
#[derive(Debug, Default)]
pub struct GeneManifestBuilder {
    id: Option<String>,
    name: Option<String>,
    kind: Option<GeneKind>,
    version: Option<String>,
    author: Option<String>,
    description: Option<String>,
    capabilities: Vec<String>,
    dependencies: Vec<String>,
    slash_commands: Vec<super::harness::SlashCommand>,
    permissions: Vec<String>,
    metadata: HashMap<String, String>,
    owner_harness: Option<String>,
}

impl GeneManifestBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn name(mut self, n: impl Into<String>) -> Self {
        self.name = Some(n.into());
        self
    }
    pub fn kind(mut self, k: GeneKind) -> Self {
        self.kind = Some(k);
        self
    }
    pub fn version(mut self, v: impl Into<String>) -> Self {
        self.version = Some(v.into());
        self
    }
    pub fn author(mut self, a: impl Into<String>) -> Self {
        self.author = Some(a.into());
        self
    }
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
    pub fn capability(mut self, c: impl Into<String>) -> Self {
        self.capabilities.push(c.into());
        self
    }
    pub fn dependency(mut self, d: impl Into<String>) -> Self {
        self.dependencies.push(d.into());
        self
    }
    pub fn slash_command(mut self, cmd: impl Into<String>, desc: impl Into<String>) -> Self {
        self.slash_commands.push(super::harness::SlashCommand {
            command: cmd.into(),
            description: desc.into(),
        });
        self
    }
    pub fn permission(mut self, p: impl Into<String>) -> Self {
        self.permissions.push(p.into());
        self
    }
    pub fn metadata(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.metadata.insert(k.into(), v.into());
        self
    }
    pub fn owner_harness(mut self, h: impl Into<String>) -> Self {
        self.owner_harness = Some(h.into());
        self
    }

    /// Consume the builder and produce a validated `GeneManifest`.
    /// Returns an error if required fields (`id`, `name`, `kind`, `version`) are missing.
    pub fn build(self) -> Result<GeneManifest, crate::PandoraError> {
        let metadata = GeneMetadata {
            description: self.description.unwrap_or_default(),
            permissions: self.permissions,
            custom: self.metadata,
            ..Default::default()
        };
        Ok(GeneManifest {
            id: self.id.ok_or("Missing: id")?,
            name: self.name.ok_or("Missing: name")?,
            kind: self.kind.ok_or("Missing: kind")?,
            version: self.version.ok_or("Missing: version")?,
            author: self.author.unwrap_or_default(),
            capabilities: self.capabilities,
            dependencies: self.dependencies,
            slash_commands: self.slash_commands,
            owner_harness: self.owner_harness,
            metadata,
        })
    }
}

// ── Gene Lineage ──

/// One entry in a gene evolution lineage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneLineageEntry {
    pub entry_id: u64,
    pub parent_id: Option<String>,
    pub mutation_desc: String,
    pub benchmark_result: Option<String>,
    pub accepted: bool,
    pub timestamp: String,
    pub gene_snapshot: Option<String>,
}

/// Tracks evolution history. Origin is immutable, local commits are the lineage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneLineage {
    pub original_package_id: String,
    pub original_version: String,
    pub original_hash: Option<String>,
    pub entries: Vec<GeneLineageEntry>,
}

impl GeneLineage {
    pub fn new(package_id: &str, version: &str) -> Self {
        Self {
            original_package_id: package_id.to_string(),
            original_version: version.to_string(),
            original_hash: None,
            entries: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, entry: GeneLineageEntry) {
        self.entries.push(entry);
    }
    pub fn latest_entry(&self) -> Option<&GeneLineageEntry> {
        self.entries.last()
    }
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

// ── Gene Trait ──

/// Canonical runtime Gene trait.
///
/// This is the primary trait developers implement when building genes.
/// Most developers should implement this trait.
///
/// # Example
///
/// ```rust,no_run
/// use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
///
/// #[derive(Debug)]
/// struct MyGene { m: GeneManifest }
/// impl Gene for MyGene {
///     fn manifest(&self) -> &GeneManifest { &self.m }
///     fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
///         Ok(format!("hello: {input}"))
///     }
/// }
/// ```
///
/// See `constitutional::Gene` only when implementing governance-level genes
/// that require access to `GeneExecutionContext` and constitutional manifest APIs.
pub trait Gene: Send + Sync + std::fmt::Debug {
    /// Access the gene's canonical manifest.
    fn manifest(&self) -> &GeneManifest;

    /// Execute the gene with the given input. Returns a JSON-serializable result.
    fn execute(&self, _input: &str) -> Result<String, crate::PandoraError> {
        Err(crate::PandoraError::gene("execute not implemented"))
    }

    /// Validate the gene's configuration.
    fn validate(&self) -> Result<(), crate::PandoraError> {
        Ok(())
    }

    // ── Convenience methods ──

    fn id(&self) -> &str {
        &self.manifest().id
    }
    fn name(&self) -> &str {
        &self.manifest().name
    }
    fn kind(&self) -> &GeneKind {
        &self.manifest().kind
    }
}

#[non_exhaustive]
/// Who owns a slash command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlashCommandOwner {
    Harness(String),
    Gene(String),
}

impl SlashCommandOwner {
    pub fn id(&self) -> &str {
        match self {
            Self::Harness(id) => id,
            Self::Gene(id) => id,
        }
    }
}
