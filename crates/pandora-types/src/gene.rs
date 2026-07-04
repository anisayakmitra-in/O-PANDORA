//! Canonical Gene types — GeneManifest (runtime) + GeneMetadata (rich) — the universal building block of Pandora.
//!
//! Genes are small composable runtime units. Every capability is a Gene.
//! They can exist alone or inside a Harness.

use serde::{Deserialize, Serialize};

/// Kind of gene — avoids 100+ special structs by using one canonical manifest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    Custom(String),
}

impl GeneKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            GeneKind::Tool => "tool",
            GeneKind::Provider => "provider",
            GeneKind::Workflow => "workflow",
            GeneKind::Agent => "agent",
            GeneKind::Skill => "skill",
            GeneKind::Memory => "memory",
            GeneKind::Planner => "planner",
            GeneKind::Reasoner => "reasoner",
            GeneKind::Execution => "execution",
            GeneKind::SlashCommand => "slash_command",
            GeneKind::MCP => "mcp",
            GeneKind::Knowledge => "knowledge",
            GeneKind::Permission => "permission",
            GeneKind::Custom(_) => "custom",
        }
    }
}

/// Canonical Gene manifest — minimal runtime fields only.
/// Rich metadata (description, homepage, license, etc.) lives in GeneMetadata.
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

/// Rich metadata for a Gene — used by KUBER for display, search, and publishing.
/// Not part of the runtime execution path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneMetadata {
    pub description: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub tags: Vec<String>,
    pub documentation: Option<String>,
    pub icon: Option<String>,
    pub examples: Vec<String>,
    pub custom: std::collections::HashMap<String, String>,
    pub permissions: Vec<String>,
}

impl GeneMetadata {
    pub fn new() -> Self {
        Self {
            description: String::new(),
            homepage: None,
            license: None,
            repository: None,
            tags: Vec::new(),
            documentation: None,
            icon: None,
            examples: Vec::new(),
            custom: std::collections::HashMap::new(),
            permissions: Vec::new(),
        }
    }
}

impl Default for GeneMetadata {
    fn default() -> Self {
        Self::new()
    }
}

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

impl GeneManifest {
    pub fn builder() -> GeneManifestBuilder {
        GeneManifestBuilder::default()
    }
}

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
    metadata: std::collections::HashMap<String, String>,
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

    pub fn build(self) -> Result<GeneManifest, String> {
        let metadata = GeneMetadata {
            description: self.description.unwrap_or_default(),
            homepage: None,
            license: None,
            repository: None,
            tags: Vec::new(),
            documentation: None,
            icon: None,
            examples: Vec::new(),
            custom: self.metadata,
            permissions: self.permissions,
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

/// Generic trait for any Gene type.
/// All gene kinds share this same runtime contract.
pub trait Gene: Send + Sync + std::fmt::Debug {
    fn manifest(&self) -> &GeneManifest;

    /// Execute the gene with the given input.
    /// Returns a JSON-serializable result.
    fn execute(&self, _input: &str) -> Result<String, String> {
        Err("execute not implemented".into())
    }

    /// Validate the gene's configuration.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    // Convenience
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

/// Who owns a slash command — Harness or Gene.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlashCommandOwner {
    Harness(String),
    Gene(String),
}

impl SlashCommandOwner {
    pub fn id(&self) -> &str {
        match self {
            SlashCommandOwner::Harness(id) => id,
            SlashCommandOwner::Gene(id) => id,
        }
    }
}
