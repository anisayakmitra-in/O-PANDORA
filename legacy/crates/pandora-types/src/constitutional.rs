//! Constitutional Manifest framework.
//!
//! Every installable, governance-aware object in Pandora derives its
//! manifest from a single `ConstitutionalManifest`. This module defines
//! the canonical base type, the supporting metadata types, and the
//! validation/registry framework that all constitutional objects use.
//!
//! Every constitutional object uses `ConstitutionalManifest` with its
//! appropriate `ManifestKind`. No separate per-kind manifest types exist.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::gene_context::{GeneExecutionContext, GeneExecutionResult};
use crate::universal::GeneManifest;
use serde::{Deserialize, Serialize};

/// The kind of constitutional object a manifest describes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ManifestKind {
    SourceHarness, MetaHarness, Gene, Provider, Tool, Loop, Mcp, Plugin,
    Package, SandboxBackend, MemoryBackend, ExecutionBackend,
    Workflow, Agent, Model, Dataset, Capability, Custom(String),
}

impl ManifestKind {
    pub fn name(&self) -> &str {
        match self {
            Self::SourceHarness => "SourceHarness", Self::MetaHarness => "MetaHarness",
            Self::Gene => "Gene", Self::Provider => "Provider", Self::Tool => "Tool",
            Self::Loop => "Loop", Self::Mcp => "MCP", Self::Plugin => "Plugin",
            Self::Package => "Package", Self::SandboxBackend => "SandboxBackend",
            Self::MemoryBackend => "MemoryBackend", Self::ExecutionBackend => "ExecutionBackend",
            Self::Workflow => "Workflow", Self::Agent => "Agent", Self::Model => "Model",
            Self::Dataset => "Dataset", Self::Capability => "Capability",
            Self::Custom(s) => s,
        }
    }
    pub fn all_known() -> &'static [ManifestKind] {
        &[Self::SourceHarness, Self::MetaHarness, Self::Gene, Self::Provider, Self::Tool, Self::Loop, Self::Mcp, Self::Plugin, Self::Package, Self::SandboxBackend, Self::MemoryBackend, Self::ExecutionBackend, Self::Workflow, Self::Agent, Self::Model, Self::Dataset, Self::Capability]
    }
}

impl fmt::Display for ManifestKind { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.name()) } }

/// The role of a Source Harness in the constitutional architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceHarnessRole { Cognition, Planning, Verification, Governance, Evolution, Execution }

impl SourceHarnessRole {
    pub fn name(&self) -> &'static str { match self { Self::Cognition => "Cognition", Self::Planning => "Planning", Self::Verification => "Verification", Self::Governance => "Governance", Self::Evolution => "Evolution", Self::Execution => "Execution" } }
}

/// Pre-freeze Source Harness trait. Use `Harness` trait in `pandora_types::harness` instead.
#[deprecated(note = "Use the single Harness trait in pandora_types::harness instead.")]
pub trait SourceHarness: Send + Sync + std::fmt::Debug {
    fn role(&self) -> SourceHarnessRole;
    fn manifest(&self) -> &ConstitutionalManifest;
    fn health(&self) -> &ManifestHealth;
    fn lifecycle(&self) -> &ManifestLifecycle;
    fn telemetry(&self) -> &ManifestTelemetry;
    fn capabilities(&self) -> &[ManifestCapability];
    fn dependencies(&self) -> &[SourceHarnessRole];
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetaHarnessKind { Security, VLSI, Robotics, Research, General, Custom(String) }

impl MetaHarnessKind {
    pub fn name(&self) -> String { match self { Self::Security => "Security".into(), Self::VLSI => "VLSI".into(), Self::Robotics => "Robotics".into(), Self::Research => "Research".into(), Self::General => "General".into(), Self::Custom(s) => s.clone() } }
}

/// Pre-freeze Meta Harness trait. Use `Harness` trait in `pandora_types::harness` instead.
#[deprecated(note = "Use the single Harness trait in pandora_types::harness instead.")]
pub trait MetaHarness: Send + Sync + std::fmt::Debug {
    fn kind(&self) -> MetaHarnessKind;
    fn manifest(&self) -> &ConstitutionalManifest;
    fn health(&self) -> &ManifestHealth;
    fn lifecycle(&self) -> &ManifestLifecycle;
    fn telemetry(&self) -> &ManifestTelemetry;
    fn capabilities(&self) -> &[ManifestCapability];
    fn parent_source_harness(&self) -> SourceHarnessRole;
}

/// Pre-freeze Gene trait. Use `Gene` trait in `pandora_types::gene` instead.
pub trait Gene: Send + Sync + std::fmt::Debug {
    fn manifest(&self) -> &GeneManifest;
    fn execute(&self, ctx: &GeneExecutionContext) -> GeneExecutionResult;
}

// ── Version & Identity ──

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestVersion { pub major: u32, pub minor: u32, pub patch: u32 }

impl ManifestVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self { Self { major, minor, patch } }
    pub fn as_string(&self) -> String { format!("{}.{}.{}", self.major, self.minor, self.patch) }
}

impl fmt::Display for ManifestVersion { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.as_string()) } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestSchemaVersion { pub major: u32, pub minor: u32 }

impl ManifestSchemaVersion {
    pub fn new(major: u32, minor: u32) -> Self { Self { major, minor } }
    pub fn as_string(&self) -> String { format!("schema/{}.{}", self.major, self.minor) }
}

impl Default for ManifestSchemaVersion { fn default() -> Self { Self::new(1, 0) } }

// ── Author / License / Repository / Homepage / Docs / Examples ──

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestAuthor { pub name: String, #[serde(default, skip_serializing_if = "Option::is_none")] pub email: Option<String>, #[serde(default, skip_serializing_if = "Option::is_none")] pub organization: Option<String> }
impl ManifestAuthor { pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), email: None, organization: None } } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestLicense { pub spdx_id: String, #[serde(default, skip_serializing_if = "Option::is_none")] pub name: Option<String> }
impl ManifestLicense { pub fn spdx(id: impl Into<String>) -> Self { Self { spdx_id: id.into(), name: None } } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestRepository { pub url: String, #[serde(default, skip_serializing_if = "Option::is_none")] pub branch: Option<String> }
impl ManifestRepository { pub fn git(url: impl Into<String>) -> Self { Self { url: url.into(), branch: None } } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestHomepage { pub url: String }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestDocumentation { pub url: String }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestExample { pub name: String, pub description: String, #[serde(default, skip_serializing_if = "Option::is_none")] pub snippet: Option<String> }
impl ManifestExample { pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self { Self { name: name.into(), description: description.into(), snippet: None } } }

// ── Capabilities / Dependencies / Relationships ──

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestCapability { pub name: String, pub description: String }
impl ManifestCapability { pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self { Self { name: name.into(), description: description.into() } } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestDependency { pub name: String, pub kind: ManifestKind, #[serde(default, skip_serializing_if = "Option::is_none")] pub version: Option<ManifestVersion> }
impl ManifestDependency {
    pub fn new(name: impl Into<String>, kind: ManifestKind) -> Self { Self { name: name.into(), kind, version: None } }
    pub fn with_version(mut self, v: ManifestVersion) -> Self { self.version = Some(v); self }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestRelationship { pub kind: String, pub target: String, #[serde(default, skip_serializing_if = "Option::is_none")] pub note: Option<String> }
impl ManifestRelationship { pub fn new(kind: impl Into<String>, target: impl Into<String>) -> Self { Self { kind: kind.into(), target: target.into(), note: None } } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestRelationshipSet { pub relationships: Vec<ManifestRelationship> }
impl ManifestRelationshipSet {
    pub fn new() -> Self { Self::default() }
    pub fn push(mut self, r: ManifestRelationship) -> Self { self.relationships.push(r); self }
}

// ── Trust / Health / Telemetry / Provenance ──

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestTrust { pub level: TrustLevel, pub verified: bool }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TrustLevel { #[default] Unknown, Untrusted, Community, Verified, Official }
impl ManifestTrust { pub fn new(level: TrustLevel) -> Self { Self { level, verified: false } } pub fn verified(mut self) -> Self { self.verified = true; self } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestHealth { pub status: String }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestTelemetry { pub counters: BTreeMap<String, u64> }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestProvenance { pub source: String, #[serde(default, skip_serializing_if = "Option::is_none")] pub fetched_at: Option<String> }
impl ManifestProvenance { pub fn from_source(source: impl Into<String>) -> Self { Self { source: source.into(), fetched_at: None } } }

// ── Lifecycle ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ManifestLifecycleState { #[default] Registered, Booting, Ready, ShuttingDown, Stopped, Failed }
impl ManifestLifecycleState { pub fn as_str(self) -> &'static str { match self { Self::Registered => "REGISTERED", Self::Booting => "BOOTING", Self::Ready => "READY", Self::ShuttingDown => "SHUTTING_DOWN", Self::Stopped => "STOPPED", Self::Failed => "FAILED" } } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestLifecycle { pub state: ManifestLifecycleState }

// ── Signature / Checksum / Compatibility / Extensions / Metadata ──

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestSignature { pub algorithm: String, pub signature: String, pub key_id: String }
impl ManifestSignature { pub fn new(algorithm: impl Into<String>, signature: impl Into<String>, key_id: impl Into<String>) -> Self { Self { algorithm: algorithm.into(), signature: signature.into(), key_id: key_id.into() } } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestChecksum { pub algorithm: String, pub value: String }
impl ManifestChecksum { pub fn new(algorithm: impl Into<String>, value: impl Into<String>) -> Self { Self { algorithm: algorithm.into(), value: value.into() } } }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestCompatibility { #[serde(default, skip_serializing_if = "Option::is_none")] pub upgrade_from: Option<ManifestVersion>, #[serde(default, skip_serializing_if = "Option::is_none")] pub downgrade_to: Option<ManifestVersion>, #[serde(default)] pub migration_notes: Vec<String> }
impl ManifestCompatibility {
    pub fn new() -> Self { Self::default() }
    pub fn upgrade_from(mut self, v: ManifestVersion) -> Self { self.upgrade_from = Some(v); self }
    pub fn downgrade_to(mut self, v: ManifestVersion) -> Self { self.downgrade_to = Some(v); self }
    pub fn with_migration_note(mut self, note: impl Into<String>) -> Self { self.migration_notes.push(note.into()); self }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestExtensions { pub values: BTreeMap<String, String> }
impl ManifestExtensions {
    pub fn new() -> Self { Self::default() }
    pub fn set(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.values.insert(key.into(), value.into()); self }
    pub fn get(&self, key: &str) -> Option<&str> { self.values.get(key).map(String::as_str) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestMetadata { pub values: BTreeMap<String, String> }
impl ManifestMetadata { pub fn new() -> Self { Self::default() } pub fn set(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.values.insert(key.into(), value.into()); self } }

// ── Identity Manifest ──

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentityManifest { pub name: String, pub kind: ManifestKind, pub version: ManifestVersion }
impl IdentityManifest {
    pub fn new(name: impl Into<String>, kind: ManifestKind, version: ManifestVersion) -> Self { Self { name: name.into(), kind, version } }
    pub fn id(&self) -> String { format!("{}@{}", self.name, self.version) }
}

// ── Constitutional Manifest ──

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstitutionalManifest {
    pub identity: IdentityManifest, pub schema_version: ManifestSchemaVersion,
    pub description: String, #[serde(default, skip_serializing_if = "Option::is_none")] pub author: Option<ManifestAuthor>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub license: Option<ManifestLicense>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub repository: Option<ManifestRepository>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub homepage: Option<ManifestHomepage>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub documentation: Option<ManifestDocumentation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub examples: Vec<ManifestExample>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub capabilities: Vec<ManifestCapability>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub dependencies: Vec<ManifestDependency>,
    #[serde(default)] pub relationships: ManifestRelationshipSet,
    pub trust: ManifestTrust, #[serde(default)] pub health: ManifestHealth,
    #[serde(default)] pub telemetry: ManifestTelemetry,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub provenance: Option<ManifestProvenance>,
    #[serde(default)] pub lifecycle: ManifestLifecycle,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub signature: Option<ManifestSignature>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub checksum: Option<ManifestChecksum>,
    #[serde(default)] pub compatibility: ManifestCompatibility,
    #[serde(default)] pub extensions: ManifestExtensions, #[serde(default)] pub metadata: ManifestMetadata,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub category: Option<String>,
}

impl ConstitutionalManifest {
    pub fn new(name: impl Into<String>, kind: ManifestKind, version: ManifestVersion, description: impl Into<String>) -> Self {
        Self { identity: IdentityManifest::new(name, kind, version), schema_version: ManifestSchemaVersion::default(), description: description.into(), author: None, license: None, repository: None, homepage: None, documentation: None, examples: Vec::new(), capabilities: Vec::new(), dependencies: Vec::new(), relationships: ManifestRelationshipSet::default(), trust: ManifestTrust::new(TrustLevel::Unknown), health: ManifestHealth::default(), telemetry: ManifestTelemetry::default(), provenance: None, lifecycle: ManifestLifecycle::default(), signature: None, checksum: None, compatibility: ManifestCompatibility::default(), extensions: ManifestExtensions::default(), metadata: ManifestMetadata::default(), tags: Vec::new(), category: None }
    }
    pub fn id(&self) -> String { self.identity.id() }
}

// ── Builder ──

pub struct ConstitutionalManifestBuilder { inner: ConstitutionalManifest }

impl ConstitutionalManifestBuilder {
    pub fn new(name: impl Into<String>, kind: ManifestKind, version: ManifestVersion) -> Self {
        Self { inner: ConstitutionalManifest::new(name, kind, version, "") }
    }
    pub fn description(mut self, d: impl Into<String>) -> Self { self.inner.description = d.into(); self }
    pub fn author(mut self, a: ManifestAuthor) -> Self { self.inner.author = Some(a); self }
    pub fn license(mut self, l: ManifestLicense) -> Self { self.inner.license = Some(l); self }
    pub fn repository(mut self, r: ManifestRepository) -> Self { self.inner.repository = Some(r); self }
    pub fn homepage(mut self, url: impl Into<String>) -> Self { self.inner.homepage = Some(ManifestHomepage { url: url.into() }); self }
    pub fn documentation(mut self, url: impl Into<String>) -> Self { self.inner.documentation = Some(ManifestDocumentation { url: url.into() }); self }
    pub fn add_capability(mut self, c: ManifestCapability) -> Self { self.inner.capabilities.push(c); self }
    pub fn add_dependency(mut self, d: ManifestDependency) -> Self { self.inner.dependencies.push(d); self }
    pub fn add_relationship(mut self, r: ManifestRelationship) -> Self { self.inner.relationships.relationships.push(r); self }
    pub fn add_tag(mut self, tag: impl Into<String>) -> Self { self.inner.tags.push(tag.into()); self }
    pub fn category(mut self, c: impl Into<String>) -> Self { self.inner.category = Some(c.into()); self }
    pub fn trust(mut self, t: ManifestTrust) -> Self { self.inner.trust = t; self }
    pub fn signature(mut self, s: ManifestSignature) -> Self { self.inner.signature = Some(s); self }
    pub fn checksum(mut self, c: ManifestChecksum) -> Self { self.inner.checksum = Some(c); self }
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.inner.extensions.values.insert(key.into(), value.into()); self }
    pub fn build(self) -> ConstitutionalManifest { self.inner }
}

// ── Validation ──

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ManifestValidationError {
    #[error("manifest name must not be empty")] EmptyName,
    #[error("manifest description must not be empty")] EmptyDescription,
    #[error("manifest version is invalid (major must be > 0)")] InvalidVersion,
    #[error("manifest schema version is unsupported (expected 1.x, got {0})")] UnsupportedSchemaVersion(u32),
    #[error("dependency references unknown kind")] InvalidDependencyKind,
    #[error("duplicate dependency name")] DuplicateDependency,
    #[error("manifest has no capabilities declared")] NoCapabilities,
    #[error("declared capability name must not be empty")] EmptyCapabilityName,
}

pub struct ManifestValidator;

impl ManifestValidator {
    pub fn new() -> Self { Self }
    pub fn validate(&self, m: &ConstitutionalManifest) -> Result<(), ManifestValidationError> {
        if m.identity.name.trim().is_empty() { return Err(ManifestValidationError::EmptyName); }
        if m.description.trim().is_empty() { return Err(ManifestValidationError::EmptyDescription); }
        if m.identity.version.major == 0 { return Err(ManifestValidationError::InvalidVersion); }
        if m.schema_version.major != 1 { return Err(ManifestValidationError::UnsupportedSchemaVersion(m.schema_version.major)); }
        for cap in &m.capabilities { if cap.name.trim().is_empty() { return Err(ManifestValidationError::EmptyCapabilityName); } }
        let mut seen = BTreeSet::new();
        for dep in &m.dependencies { if !seen.insert(dep.name.clone()) { return Err(ManifestValidationError::DuplicateDependency); } }
        Ok(())
    }
}
impl Default for ManifestValidator { fn default() -> Self { Self::new() } }

// ── Serialization ──

pub trait ManifestSerializer: Send + Sync { fn format_name(&self) -> &str; fn serialize(&self, m: &ConstitutionalManifest) -> Result<String, String>; }
pub trait ManifestDeserializer: Send + Sync { fn format_name(&self) -> &str; fn deserialize(&self, s: &str) -> Result<ConstitutionalManifest, String>; }

pub struct JsonManifestSerializer;
impl JsonManifestSerializer { pub fn new() -> Self { Self } }
impl Default for JsonManifestSerializer { fn default() -> Self { Self::new() } }
impl ManifestSerializer for JsonManifestSerializer {
    fn format_name(&self) -> &str { "json" }
    fn serialize(&self, m: &ConstitutionalManifest) -> Result<String, String> { serde_json::to_string_pretty(m).map_err(|e| e.to_string()) }
}
impl ManifestDeserializer for JsonManifestSerializer {
    fn format_name(&self) -> &str { "json" }
    fn deserialize(&self, s: &str) -> Result<ConstitutionalManifest, String> { serde_json::from_str(s).map_err(|e| e.to_string()) }
}

pub struct YamlManifestSerializer;
impl YamlManifestSerializer { pub fn new() -> Self { Self } }
impl Default for YamlManifestSerializer { fn default() -> Self { Self::new() } }
impl ManifestSerializer for YamlManifestSerializer {
    fn format_name(&self) -> &str { "yaml" }
    fn serialize(&self, _m: &ConstitutionalManifest) -> Result<String, String> { Err("YAML support requires the serde_yaml feature".into()) }
}
impl ManifestDeserializer for YamlManifestSerializer {
    fn format_name(&self) -> &str { "yaml" }
    fn deserialize(&self, _s: &str) -> Result<ConstitutionalManifest, String> { Err("YAML support requires the serde_yaml feature".into()) }
}

pub struct TomlManifestSerializer;
impl TomlManifestSerializer { pub fn new() -> Self { Self } }
impl Default for TomlManifestSerializer { fn default() -> Self { Self::new() } }
impl ManifestSerializer for TomlManifestSerializer {
    fn format_name(&self) -> &str { "toml" }
    fn serialize(&self, _m: &ConstitutionalManifest) -> Result<String, String> { Err("TOML support requires the toml feature".into()) }
}
impl ManifestDeserializer for TomlManifestSerializer {
    fn format_name(&self) -> &str { "toml" }
    fn deserialize(&self, _s: &str) -> Result<ConstitutionalManifest, String> { Err("TOML support requires the toml feature".into()) }
}

// ── Loading & Registry ──

pub trait ManifestLoader: Send + Sync { fn loader_name(&self) -> &str; fn load(&self) -> Vec<ConstitutionalManifest>; }

pub struct InMemoryManifestLoader { name: String, manifests: Vec<ConstitutionalManifest> }
impl InMemoryManifestLoader {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), manifests: Vec::new() } }
    pub fn push(mut self, m: ConstitutionalManifest) -> Self { self.manifests.push(m); self }
}
impl ManifestLoader for InMemoryManifestLoader {
    fn loader_name(&self) -> &str { &self.name }
    fn load(&self) -> Vec<ConstitutionalManifest> { self.manifests.clone() }
}

pub struct ManifestRegistry { inner: std::sync::RwLock<Vec<ConstitutionalManifest>> }

impl ManifestRegistry {
    pub fn new() -> Self { Self { inner: std::sync::RwLock::new(Vec::new()) } }
    pub fn register(&self, m: ConstitutionalManifest) { self.inner.write().expect("registry poisoned").push(m); }
    pub fn unregister(&self, name: &str) -> bool { let mut g = self.inner.write().expect("registry poisoned"); let b = g.len(); g.retain(|m| m.identity.name != name); g.len() != b }
    pub fn lookup(&self, name: &str) -> Option<ConstitutionalManifest> { self.inner.read().expect("registry poisoned").iter().find(|m| m.identity.name == name).cloned() }
    pub fn lookup_by_identity(&self, identity: &IdentityManifest) -> Option<ConstitutionalManifest> { self.inner.read().expect("registry poisoned").iter().find(|m| m.identity.name == identity.name && m.identity.version == identity.version).cloned() }
    pub fn lookup_by_kind(&self, kind: &ManifestKind) -> Vec<ConstitutionalManifest> { self.inner.read().expect("registry poisoned").iter().filter(|m| &m.identity.kind == kind).cloned().collect() }
    pub fn lookup_by_version(&self, name: &str, version: &ManifestVersion) -> Option<ConstitutionalManifest> { self.inner.read().expect("registry poisoned").iter().find(|m| m.identity.name == name && &m.identity.version == version).cloned() }
    pub fn lookup_by_tag(&self, tag: &str) -> Vec<ConstitutionalManifest> { self.inner.read().expect("registry poisoned").iter().filter(|m| m.tags.iter().any(|t| t == tag)).cloned().collect() }
    pub fn lookup_by_category(&self, category: &str) -> Vec<ConstitutionalManifest> { self.inner.read().expect("registry poisoned").iter().filter(|m| m.category.as_deref() == Some(category)).cloned().collect() }
    pub fn validate(&self) -> Vec<(String, ManifestValidationError)> { let g = self.inner.read().expect("registry poisoned"); let v = ManifestValidator::new(); g.iter().filter_map(|m| v.validate(m).err().map(|e| (m.identity.name.clone(), e))).collect() }
    pub fn list(&self) -> Vec<ConstitutionalManifest> { self.inner.read().expect("registry poisoned").clone() }
    pub fn len(&self) -> usize { self.inner.read().expect("registry poisoned").len() }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
impl Default for ManifestRegistry { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> ConstitutionalManifest {
        ConstitutionalManifestBuilder::new("phoenix", ManifestKind::SourceHarness, ManifestVersion::new(1, 0, 0))
            .description("Execution source harness").author(ManifestAuthor::new("Arka")).license(ManifestLicense::spdx("MIT"))
            .repository(ManifestRepository::git("https://example.com/phoenix")).homepage("https://example.com/phoenix")
            .documentation("https://docs.example.com/phoenix")
            .add_capability(ManifestCapability::new("execution", "runs code")).add_capability(ManifestCapability::new("sandbox", "isolated execution"))
            .add_dependency(ManifestDependency::new("anubis", ManifestKind::SourceHarness).with_version(ManifestVersion::new(1, 0, 0)))
            .add_relationship(ManifestRelationship::new("depends-on", "anubis")).add_tag("execution").add_tag("core")
            .category("execution-harness").trust(ManifestTrust::new(TrustLevel::Official).verified())
            .signature(ManifestSignature::new("ed25519", "abc123", "key-001")).checksum(ManifestChecksum::new("sha256", "deadbeef")).build()
    }

    #[test] fn manifest_identity_id() { assert_eq!(fixture().id(), "phoenix@1.0.0"); }
    #[test] fn manifest_version_string() { assert_eq!(ManifestVersion::new(1, 2, 3).as_string(), "1.2.3"); }
    #[test] fn manifest_kind_names() { assert_eq!(ManifestKind::SourceHarness.name(), "SourceHarness"); assert_eq!(ManifestKind::Custom("x".into()).name(), "x"); }
    #[test] fn json_roundtrip() { let m = fixture(); let s = JsonManifestSerializer::new().serialize(&m).unwrap(); assert_eq!(m, JsonManifestSerializer::new().deserialize(&s).unwrap()); }
    #[test] fn validator_accepts_valid() { assert!(ManifestValidator::new().validate(&fixture()).is_ok()); }
    #[test] fn validator_rejects_empty_name() { let mut m = fixture(); m.identity.name = "   ".into(); assert!(ManifestValidator::new().validate(&m).is_err()); }
    #[test] fn validator_rejects_zero_major() { let mut m = fixture(); m.identity.version = ManifestVersion::new(0, 1, 0); assert!(ManifestValidator::new().validate(&m).is_err()); }
    #[test] fn registry_register_and_lookup() { let r = ManifestRegistry::new(); let m = fixture(); r.register(m.clone()); assert_eq!(r.lookup("phoenix"), Some(m)); }
    #[test] fn registry_unregister() { let r = ManifestRegistry::new(); r.register(fixture()); assert!(r.unregister("phoenix")); assert_eq!(r.len(), 0); }
    #[test] fn compatibility_builder() { let c = ManifestCompatibility::new().upgrade_from(ManifestVersion::new(0, 9, 0)).with_migration_note("see changelog"); assert_eq!(c.upgrade_from, Some(ManifestVersion::new(0, 9, 0))); }
    #[test] fn extensions_set_get() { let e = ManifestExtensions::new().set("k", "v").set("k2", "v2"); assert_eq!(e.get("k"), Some("v")); }
    #[test] fn trust_levels() { let t = ManifestTrust::new(TrustLevel::Official).verified(); assert!(t.verified); assert_eq!(t.level, TrustLevel::Official); }
    #[test] fn yaml_unavailable() { assert!(YamlManifestSerializer::new().serialize(&fixture()).is_err()); }
    #[test] fn toml_unavailable() { assert!(TomlManifestSerializer::new().serialize(&fixture()).is_err()); }
}
