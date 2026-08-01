//! Harness types — Source, Meta, and Domain harnesses share the same
//! lifecycle and manifest format. The `Harness` trait is the second
//! primary extension API (alongside `Gene`).

use serde::{Deserialize, Serialize};

/// Canonical kind of harness — all three share the same lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum HarnessKind {
    /// Augments one or more constitutional services.
    #[serde(alias = "source")]
    Source,
    /// Communication/orchestration mesh between harnesses.
    #[serde(alias = "meta")]
    Meta,
    /// Packages policies, workflows, capabilities, and genes for a domain.
    #[serde(alias = "domain")]
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
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub owned_genes: Vec<String>,
    #[serde(default)]
    pub slash_commands: Vec<SlashCommand>,
}

impl HarnessManifest {
    pub fn builder() -> HarnessManifestBuilder {
        HarnessManifestBuilder::default()
    }
}

/// An installable harness package — manifest + distribution metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessPackage {
    /// Canonical manifest (id, name, version, kind, capabilities, etc.)
    pub manifest: HarnessManifest,
    /// Rich metadata for display.
    pub metadata: HarnessMetadata,
    /// Optional class label (e.g., "alternative-planner", "swarm")
    pub class: Option<String>,
    /// Dependencies on other packages.
    pub dependencies: Vec<String>,
    /// Packages this one conflicts with.
    pub conflicts: Vec<String>,
    /// Source location (path, git URL, K-O-Palace id).
    pub source: String,
    /// Ed25519 signature (if signed).
    pub signature: Option<String>,
    /// Installation timestamp.
    pub installed_at: Option<String>,
    /// Whether the harness is currently enabled.
    pub enabled: bool,
}

/// Rich metadata for display and distribution — not used at runtime.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HarnessMetadata {
    pub description: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub tags: Vec<String>,
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
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
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
    pub fn kind(mut self, k: HarnessKind) -> Self {
        self.kind = Some(k);
        self
    }
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.metadata.description = d.into();
        self
    }
    pub fn dependency(mut self, dep: impl Into<String>) -> Self {
        self.dependencies.push(dep.into());
        self
    }
    pub fn capability(mut self, cap: impl Into<String>) -> Self {
        self.capabilities.push(cap.into());
        self
    }
    pub fn owned_gene(mut self, g: impl Into<String>) -> Self {
        self.owned_genes.push(g.into());
        self
    }
    pub fn slash_command(mut self, cmd: impl Into<String>, desc: impl Into<String>) -> Self {
        self.slash_commands.push(SlashCommand {
            command: cmd.into(),
            description: desc.into(),
        });
        self
    }

    /// Consume builder and produce a validated manifest.
    pub fn build(self) -> Result<HarnessManifest, crate::PandoraError> {
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
    fn initialize(&mut self) -> Result<(), crate::PandoraError> {
        Ok(())
    }
    /// Shutdown — called on uninstall or disable.
    fn shutdown(&mut self) -> Result<(), crate::PandoraError> {
        Ok(())
    }
    /// Health check.
    fn health(&self) -> Result<(), crate::PandoraError> {
        Ok(())
    }

    // ── Convenience accessors ──
    fn id(&self) -> &str {
        &self.manifest().id
    }
    fn name(&self) -> &str {
        &self.manifest().name
    }
    fn kind(&self) -> &HarnessKind {
        &self.manifest().kind
    }
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
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }
    pub fn allowed_tool(mut self, tool: impl Into<String>) -> Self {
        self.allowed_tools.push(tool.into());
        self
    }
    pub fn allowed_tools(mut self, tools: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.allowed_tools
            .extend(tools.into_iter().map(|t| t.into()));
        self
    }
    pub fn max_steps(mut self, steps: u32) -> Self {
        self.max_steps = Some(steps);
        self
    }
    pub fn requires_validation(mut self, requires: bool) -> Self {
        self.requires_validation = Some(requires);
        self
    }

    /// Generate a harness.toml from a HarnessPackage for scaffolding.
    pub fn generate_harness_toml(pkg: &HarnessPackage) -> String {
        let mut lines = Vec::new();
        lines.push(format!("id = \"{}\"", pkg.manifest.id));
        lines.push(format!("name = \"{}\"", pkg.manifest.name));
        lines.push(format!("version = \"{}\"", pkg.manifest.version));
        lines.push(format!("author = \"{}\"", pkg.manifest.author));
        lines.push(format!("kind = \"{}\"", pkg.manifest.kind.as_str()));
        if let Some(ref class) = pkg.class {
            lines.push(format!("class = \"{}\"", class));
        }
        lines.push(format!("description = \"{}\"", pkg.metadata.description));
        if !pkg.manifest.capabilities.is_empty() {
            lines.push(format!(
                "capabilities = [{}]",
                pkg.manifest
                    .capabilities
                    .iter()
                    .map(|c| format!("\"{}\"", c))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !pkg.manifest.owned_genes.is_empty() {
            lines.push(format!(
                "owned_genes = [{}]",
                pkg.manifest
                    .owned_genes
                    .iter()
                    .map(|g| format!("\"{}\"", g))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !pkg.dependencies.is_empty() {
            lines.push(format!(
                "dependencies = [{}]",
                pkg.dependencies
                    .iter()
                    .map(|d| format!("\"{}\"", d))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        lines.join("\n")
    }

    pub fn build(self) -> Result<HarnessSpec, crate::PandoraError> {
        Ok(HarnessSpec {
            name: self.name.ok_or("Missing: name")?,
            domain: self.domain.ok_or("Missing: domain")?,
            allowed_tools: self.allowed_tools,
            max_steps: self.max_steps.ok_or("Missing: max_steps")?,
            requires_validation: self
                .requires_validation
                .ok_or("Missing: requires_validation")?,
        })
    }
}

/// Generate a harness.toml from a HarnessPackage for scaffolding.
pub fn generate_harness_toml(pkg: &HarnessPackage) -> String {
    let mut lines = Vec::new();
    lines.push(format!("id = \"{}\"", pkg.manifest.id));
    lines.push(format!("name = \"{}\"", pkg.manifest.name));
    lines.push(format!("version = \"{}\"", pkg.manifest.version));
    lines.push(format!("author = \"{}\"", pkg.manifest.author));
    lines.push(format!("kind = \"{}\"", pkg.manifest.kind.as_str()));
    if let Some(ref class) = pkg.class {
        lines.push(format!("class = \"{}\"", class));
    }
    lines.push(format!("description = \"{}\"", pkg.metadata.description));
    if !pkg.manifest.capabilities.is_empty() {
        lines.push(format!(
            "capabilities = [{}]",
            pkg.manifest
                .capabilities
                .iter()
                .map(|c| format!("\"{}\"", c))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !pkg.manifest.owned_genes.is_empty() {
        lines.push(format!(
            "owned_genes = [{}]",
            pkg.manifest
                .owned_genes
                .iter()
                .map(|g| format!("\"{}\"", g))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !pkg.dependencies.is_empty() {
        lines.push(format!(
            "dependencies = [{}]",
            pkg.dependencies
                .iter()
                .map(|d| format!("\"{}\"", d))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !pkg.manifest.slash_commands.is_empty() {
        for cmd in &pkg.manifest.slash_commands {
            lines.push("[[slash_commands]]".to_string());
            lines.push(format!("command = \"{}\"", cmd.command));
            lines.push(format!("description = \"{}\"", cmd.description));
        }
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_scaffolded_lowercase_manifests() {
        for (kind, expected) in [
            ("source", HarnessKind::Source),
            ("meta", HarnessKind::Meta),
            ("domain", HarnessKind::Domain),
        ] {
            let manifest: HarnessManifest = toml::from_str(&format!(
                "id = \"test\"\nname = \"Test\"\nversion = \"1.0.0\"\nauthor = \"pandora\"\nkind = \"{kind}\""
            ))
            .expect("scaffolded manifest");

            assert_eq!(manifest.kind, expected);
            assert!(manifest.dependencies.is_empty());
            assert!(manifest.capabilities.is_empty());
            assert!(manifest.owned_genes.is_empty());
            assert!(manifest.slash_commands.is_empty());
        }
    }
}
