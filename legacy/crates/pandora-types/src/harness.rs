//! Harness types — Source, Meta, and Domain harnesses share the same
//! lifecycle and manifest format. The `Harness` trait is the second
//! primary extension API (alongside `Gene`).

use serde::{Deserialize, Serialize};

/// Canonical kind of harness — all three share the same lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HarnessKind {
    /// Augments one or more constitutional services.
    Source,
    /// Communication/orchestration mesh between harnesses.
    Meta,
    /// Packages policies, workflows, capabilities, and genes for a domain.
    Domain,
}

impl HarnessKind {
    /// Canonical string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Meta => "meta",
            Self::Domain => "domain",
        }
    }
}

/// A slash command exposed by a harness or gene.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashCommand {
    pub command: String,
    pub description: String,
}

/// Canonical manifest for any harness type — minimal runtime fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub kind: HarnessKind,
    pub dependencies: Vec<String>,
    pub capabilities: Vec<String>,
    pub owned_genes: Vec<String>,
    pub slash_commands: Vec<SlashCommand>,
}

impl HarnessManifest {
    pub fn builder() -> HarnessManifestBuilder {
        HarnessManifestBuilder::default()
    }
}

/// Rich metadata for display and distribution — not used at runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessMetadata {
    pub description: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub tags: Vec<String>,
}

impl Default for HarnessMetadata {
    fn default() -> Self {
        Self {
            description: String::new(),
            homepage: None,
            license: None,
            tags: Vec::new(),
        }
    }
}

// ── Harness Manifest Builder ──

#[derive(Debug, Default)]
pub struct HarnessManifestBuilder {
    id: Option<String>,
    name: Option<String>,
    version: Option<String>,
    author: Option<String>,
    kind: Option<HarnessKind>,
    metadata: HarnessMetadata,
    dependencies: Vec<String>,
    capabilities: Vec<String>,
    owned_genes: Vec<String>,
    slash_commands: Vec<SlashCommand>,
}

impl HarnessManifestBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = Some(id.into()); self }
    pub fn name(mut self, name: impl Into<String>) -> Self { self.name = Some(name.into()); self }
    pub fn version(mut self, v: impl Into<String>) -> Self { self.version = Some(v.into()); self }
    pub fn author(mut self, a: impl Into<String>) -> Self { self.author = Some(a.into()); self }
    pub fn kind(mut self, k: HarnessKind) -> Self { self.kind = Some(k); self }
    pub fn description(mut self, d: impl Into<String>) -> Self { self.metadata.description = d.into(); self }
    pub fn dependency(mut self, dep: impl Into<String>) -> Self { self.dependencies.push(dep.into()); self }
    pub fn capability(mut self, cap: impl Into<String>) -> Self { self.capabilities.push(cap.into()); self }
    pub fn owned_gene(mut self, g: impl Into<String>) -> Self { self.owned_genes.push(g.into()); self }
    pub fn slash_command(mut self, cmd: impl Into<String>, desc: impl Into<String>) -> Self {
        self.slash_commands.push(SlashCommand { command: cmd.into(), description: desc.into() });
        self
    }

    /// Consume builder and produce a validated manifest.
    pub fn build(self) -> Result<HarnessManifest, String> {
        Ok(HarnessManifest {
            id: self.id.ok_or("Missing required field: id")?,
            name: self.name.ok_or("Missing required field: name")?,
            version: self.version.ok_or("Missing required field: version")?,
            author: self.author.ok_or("Missing required field: author")?,
            kind: self.kind.ok_or("Missing required field: kind")?,
            dependencies: self.dependencies,
            capabilities: self.capabilities,
            owned_genes: self.owned_genes,
            slash_commands: self.slash_commands,
        })
    }
}

/// Generic trait for any harness type.
///
/// Source, Meta, and Domain harnesses all implement this trait.
/// The `Harness` trait is the second primary extension API (alongside `Gene`).
pub trait Harness: Send + Sync + std::fmt::Debug {
    fn manifest(&self) -> &HarnessManifest;

    /// Initialize — called on install or load.
    fn initialize(&mut self) -> Result<(), String> { Ok(()) }
    /// Shutdown — called on uninstall or disable.
    fn shutdown(&mut self) -> Result<(), String> { Ok(()) }
    /// Health check.
    fn health(&self) -> Result<(), String> { Ok(()) }

    // ── Convenience accessors ──
    fn id(&self) -> &str { &self.manifest().id }
    fn name(&self) -> &str { &self.manifest().name }
    fn kind(&self) -> &HarnessKind { &self.manifest().kind }
}

// ── HarnessSpec (backward compat) ──

/// Configuration spec for a harness — used for initialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessSpec {
    pub name: String,
    pub domain: String,
    pub allowed_tools: Vec<String>,
    pub max_steps: u32,
    pub requires_validation: bool,
}

impl HarnessSpec {
    pub fn builder() -> HarnessSpecBuilder {
        HarnessSpecBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct HarnessSpecBuilder {
    name: Option<String>,
    domain: Option<String>,
    allowed_tools: Vec<String>,
    max_steps: Option<u32>,
    requires_validation: Option<bool>,
}

impl HarnessSpecBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self { self.name = Some(name.into()); self }
    pub fn domain(mut self, domain: impl Into<String>) -> Self { self.domain = Some(domain.into()); self }
    pub fn allowed_tool(mut self, tool: impl Into<String>) -> Self { self.allowed_tools.push(tool.into()); self }
    pub fn allowed_tools(mut self, tools: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.allowed_tools.extend(tools.into_iter().map(|t| t.into()));
        self
    }
    pub fn max_steps(mut self, steps: u32) -> Self { self.max_steps = Some(steps); self }
    pub fn requires_validation(mut self, requires: bool) -> Self { self.requires_validation = Some(requires); self }
    pub fn build(self) -> Result<HarnessSpec, String> {
        Ok(HarnessSpec {
            name: self.name.ok_or("Missing: name")?,
            domain: self.domain.ok_or("Missing: domain")?,
            allowed_tools: self.allowed_tools,
            max_steps: self.max_steps.ok_or("Missing: max_steps")?,
            requires_validation: self.requires_validation.ok_or("Missing: requires_validation")?,
        })
    }
}
