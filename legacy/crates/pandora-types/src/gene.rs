//! Canonical Gene types — the universal building block of Pandora.
//!
//! Genes are small composable runtime units. Every capability is a Gene.
//! They can exist alone or inside a Harness. The `Gene` trait is the
//! primary extension API for Pandora.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Gene Kind ──

/// Classification of a gene's purpose.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum GeneKind {
    Tool,
    Provider,
    Workflow,
    Agent,
    Skill,
    Memory,
    Planner,
    Reasoner,
    Execution,
    SlashCommand,
    MCP,
    Knowledge,
    Permission,
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
            Self::Agent => "agent",
            Self::Skill => "skill",
            Self::Memory => "memory",
            Self::Planner => "planner",
            Self::Reasoner => "reasoner",
            Self::Execution => "execution",
            Self::SlashCommand => "slash_command",
            Self::MCP => "mcp",
            Self::Knowledge => "knowledge",
            Self::Permission => "permission",
            Self::Benchmark => "benchmark",
            Self::Custom(_) => "custom",
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
    pub fn build(self) -> Result<GeneManifest, String> {
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
/// struct MyGene { m: GeneManifest }
/// impl Gene for MyGene {
///     fn manifest(&self) -> &GeneManifest { &self.m }
///     fn execute(&self, input: &str) -> Result<String, String> {
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
    fn execute(&self, _input: &str) -> Result<String, String> {
        Err("execute not implemented".into())
    }

    /// Validate the gene's configuration.
    fn validate(&self) -> Result<(), String> {
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
