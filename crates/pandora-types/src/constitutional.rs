//! Constitutional Manifest framework.
//!
//! Every installable, governance-aware object in Pandora
//! derives its manifest from a single .
//! This module defines the canonical base type, the
//! supporting metadata types, and the validation/registry
//! framework that all constitutional objects use.
//!
//! ## Architecture
//!
//! ConstitutionalManifest
//!     |
//!     +-- IdentityManifest   (who + which kind)
//!     +-- Version + SchemaVersion
//!     +-- Author / License / Repository / Homepage
//!     +-- Documentation / Examples
//!     +-- Capabilities / Dependencies
//!     +-- ManifestRelationshipSet
//!     +-- Trust / Health / Telemetry / Provenance
//!     +-- ManifestLifecycle
//!     +-- ManifestSignature / ManifestChecksum
//!     +-- Tags / Category / Metadata
//!
//! Every concrete manifest (SourceHarnessManifest,
//! MetaHarnessManifest, GeneManifest, ...) embeds (or
//! composes) a . Future
//! manifests (Providers, Tools, Loops, MCPs, Plugins,
//! Packages, Workflows, Agents) follow the same
//! pattern without any change to this base.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// The kind of constitutional object a manifest
/// describes. New kinds (Provider, Tool, Loop, MCP,
/// Plugin, Package, Workflow, Agent, Model, Dataset,
/// Capability, SandboxBackend, MemoryBackend,
/// ExecutionBackend) are added by extending this enum
/// or by using the  variant with a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ManifestKind {
    SourceHarness,
    MetaHarness,
    Gene,
    Provider,
    Tool,
    Loop,
    Mcp,
    Plugin,
    Package,
    SandboxBackend,
    MemoryBackend,
    ExecutionBackend,
    Workflow,
    Agent,
    Model,
    Dataset,
    Capability,
    Custom(String),
}

impl ManifestKind {
    pub fn name(&self) -> &str {
        match self {
            ManifestKind::SourceHarness => "SourceHarness",
            ManifestKind::MetaHarness => "MetaHarness",
            ManifestKind::Gene => "Gene",
            ManifestKind::Provider => "Provider",
            ManifestKind::Tool => "Tool",
            ManifestKind::Loop => "Loop",
            ManifestKind::Mcp => "MCP",
            ManifestKind::Plugin => "Plugin",
            ManifestKind::Package => "Package",
            ManifestKind::SandboxBackend => "SandboxBackend",
            ManifestKind::MemoryBackend => "MemoryBackend",
            ManifestKind::ExecutionBackend => "ExecutionBackend",
            ManifestKind::Workflow => "Workflow",
            ManifestKind::Agent => "Agent",
            ManifestKind::Model => "Model",
            ManifestKind::Dataset => "Dataset",
            ManifestKind::Capability => "Capability",
            ManifestKind::Custom(s) => s,
        }
    }

    pub fn all_known() -> &'static [ManifestKind] {
        &[
            ManifestKind::SourceHarness,
            ManifestKind::MetaHarness,
            ManifestKind::Gene,
            ManifestKind::Provider,
            ManifestKind::Tool,
            ManifestKind::Loop,
            ManifestKind::Mcp,
            ManifestKind::Plugin,
            ManifestKind::Package,
            ManifestKind::SandboxBackend,
            ManifestKind::MemoryBackend,
            ManifestKind::ExecutionBackend,
            ManifestKind::Workflow,
            ManifestKind::Agent,
            ManifestKind::Model,
            ManifestKind::Dataset,
            ManifestKind::Capability,
        ]
    }
}

impl fmt::Display for ManifestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A version. Wraps semver-like semantics: major,
/// minor, patch. Used for both the manifest's own
/// version and the schema version it conforms to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ManifestVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        ManifestVersion {
            major,
            minor,
            patch,
        }
    }

    pub fn as_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl fmt::Display for ManifestVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_string())
    }
}

/// The schema version the manifest conforms to. The
/// schema is the *shape* of the manifest; the version
/// is the *content* version. They evolve independently.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestSchemaVersion {
    pub major: u32,
    pub minor: u32,
}

impl ManifestSchemaVersion {
    pub fn new(major: u32, minor: u32) -> Self {
        ManifestSchemaVersion { major, minor }
    }

    pub fn as_string(&self) -> String {
        format!("schema/{}.{}", self.major, self.minor)
    }
}

impl Default for ManifestSchemaVersion {
    fn default() -> Self {
        ManifestSchemaVersion::new(1, 0)
    }
}

/// Author / authorship information. Free-form;
/// KUBER Palace may enrich this with signing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestAuthor {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}

impl ManifestAuthor {
    pub fn new(name: impl Into<String>) -> Self {
        ManifestAuthor {
            name: name.into(),
            email: None,
            organization: None,
        }
    }
}

/// License information. SPDX-style identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestLicense {
    pub spdx_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ManifestLicense {
    pub fn spdx(id: impl Into<String>) -> Self {
        ManifestLicense {
            spdx_id: id.into(),
            name: None,
        }
    }
}

/// Repository information.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestRepository {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

impl ManifestRepository {
    pub fn git(url: impl Into<String>) -> Self {
        ManifestRepository {
            url: url.into(),
            branch: None,
        }
    }
}

/// Homepage.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestHomepage {
    pub url: String,
}

/// Documentation entrypoint.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestDocumentation {
    pub url: String,
}

/// An example demonstrating use of the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestExample {
    pub name: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

impl ManifestExample {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        ManifestExample {
            name: name.into(),
            description: description.into(),
            snippet: None,
        }
    }
}

/// A capability declaration. Constitutional objects
/// declare what they can do.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestCapability {
    pub name: String,
    pub description: String,
}

impl ManifestCapability {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        ManifestCapability {
            name: name.into(),
            description: description.into(),
        }
    }
}

/// A dependency on another constitutional object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestDependency {
    pub name: String,
    pub kind: ManifestKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ManifestVersion>,
}

impl ManifestDependency {
    pub fn new(name: impl Into<String>, kind: ManifestKind) -> Self {
        ManifestDependency {
            name: name.into(),
            kind,
            version: None,
        }
    }

    pub fn with_version(mut self, v: ManifestVersion) -> Self {
        self.version = Some(v);
        self
    }
}

/// A relationship between two constitutional objects.
/// Examples: "extends", "depends-on", "replaces",
/// "conflicts-with".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestRelationship {
    pub kind: String,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl ManifestRelationship {
    pub fn new(kind: impl Into<String>, target: impl Into<String>) -> Self {
        ManifestRelationship {
            kind: kind.into(),
            target: target.into(),
            note: None,
        }
    }
}

/// A set of relationships.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestRelationshipSet {
    pub relationships: Vec<ManifestRelationship>,
}

impl ManifestRelationshipSet {
    pub fn new() -> Self {
        ManifestRelationshipSet::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, r: ManifestRelationship) -> Self {
        self.relationships.push(r);
        self
    }
}

/// Trust metadata. Records provenance, signature,
/// checksum, and trust level.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestTrust {
    pub level: TrustLevel,
    pub verified: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TrustLevel {
    #[default]
    Unknown,
    Untrusted,
    Community,
    Verified,
    Official,
}

impl ManifestTrust {
    pub fn new(level: TrustLevel) -> Self {
        ManifestTrust {
            level,
            verified: false,
        }
    }

    pub fn verified(mut self) -> Self {
        self.verified = true;
        self
    }
}

/// Health metadata (status string). The runtime fills
/// this in as the constitutional object runs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestHealth {
    pub status: String,
}

/// Telemetry metadata (counters). The runtime fills
/// this in.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestTelemetry {
    pub counters: BTreeMap<String, u64>,
}

/// Provenance metadata. Records where the manifest
/// came from.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestProvenance {
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetched_at: Option<String>,
}

impl ManifestProvenance {
    pub fn from_source(source: impl Into<String>) -> Self {
        ManifestProvenance {
            source: source.into(),
            fetched_at: None,
        }
    }
}

/// Lifecycle state. Mirrors the  from
/// the runtime module but is more abstract for any
/// constitutional object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ManifestLifecycleState {
    #[default]
    Registered,
    Booting,
    Ready,
    ShuttingDown,
    Stopped,
    Failed,
}

impl ManifestLifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            ManifestLifecycleState::Registered => "REGISTERED",
            ManifestLifecycleState::Booting => "BOOTING",
            ManifestLifecycleState::Ready => "READY",
            ManifestLifecycleState::ShuttingDown => "SHUTTING_DOWN",
            ManifestLifecycleState::Stopped => "STOPPED",
            ManifestLifecycleState::Failed => "FAILED",
        }
    }
}

/// Lifecycle metadata bundle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestLifecycle {
    pub state: ManifestLifecycleState,
}

/// Manifest signature. Records the algorithm and the
/// signature bytes. Cryptography is not implemented
/// here; this is the contract only.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestSignature {
    pub algorithm: String,
    pub signature: String,
    pub key_id: String,
}

impl ManifestSignature {
    pub fn new(
        algorithm: impl Into<String>,
        signature: impl Into<String>,
        key_id: impl Into<String>,
    ) -> Self {
        ManifestSignature {
            algorithm: algorithm.into(),
            signature: signature.into(),
            key_id: key_id.into(),
        }
    }
}

/// Manifest checksum. Records the algorithm and the
/// checksum bytes. Cryptography is not implemented
/// here; this is the contract only.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestChecksum {
    pub algorithm: String,
    pub value: String,
}

impl ManifestChecksum {
    pub fn new(algorithm: impl Into<String>, value: impl Into<String>) -> Self {
        ManifestChecksum {
            algorithm: algorithm.into(),
            value: value.into(),
        }
    }
}

/// Compatibility information. Records what versions
/// of *this* manifest are compatible with this one.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestCompatibility {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upgrade_from: Option<ManifestVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_to: Option<ManifestVersion>,
    #[serde(default)]
    pub migration_notes: Vec<String>,
}

impl ManifestCompatibility {
    pub fn new() -> Self {
        ManifestCompatibility::default()
    }

    pub fn upgrade_from(mut self, v: ManifestVersion) -> Self {
        self.upgrade_from = Some(v);
        self
    }

    pub fn downgrade_to(mut self, v: ManifestVersion) -> Self {
        self.downgrade_to = Some(v);
        self
    }

    pub fn with_migration_note(mut self, note: impl Into<String>) -> Self {
        self.migration_notes.push(note.into());
        self
    }
}

/// Manifest extensions. Free-form key-value pairs for
/// constitutional-object-specific metadata that
/// doesn't belong in the canonical fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestExtensions {
    pub values: BTreeMap<String, String>,
}

impl ManifestExtensions {
    pub fn new() -> Self {
        ManifestExtensions::default()
    }

    pub fn set(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.values.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }
}

/// Free-form metadata. Last-resort bucket for
/// constitutional-object-specific structured data
/// that the canonical fields don't cover.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ManifestMetadata {
    pub values: BTreeMap<String, String>,
}

impl ManifestMetadata {
    pub fn new() -> Self {
        ManifestMetadata::default()
    }

    pub fn set(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.values.insert(key.into(), value.into());
        self
    }
}

/// The identity portion of a .
/// This is the part the registry indexes by.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentityManifest {
    pub name: String,
    pub kind: ManifestKind,
    pub version: ManifestVersion,
}

impl IdentityManifest {
    pub fn new(name: impl Into<String>, kind: ManifestKind, version: ManifestVersion) -> Self {
        IdentityManifest {
            name: name.into(),
            kind,
            version,
        }
    }

    /// Returns a string id suitable for registry
    /// indexing. Format: "<name>@<version>".
    pub fn id(&self) -> String {
        format!("{}@{}", self.name, self.version)
    }
}

/// The constitutional manifest. Every installable,
/// governance-aware object in Pandora composes or
/// derives from this struct. The framework is the
/// canonical metadata foundation.
///
/// New constitutional objects extend the
///  enum or use  and
/// compose this struct into their own descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstitutionalManifest {
    pub identity: IdentityManifest,
    pub schema_version: ManifestSchemaVersion,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<ManifestAuthor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<ManifestLicense>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<ManifestRepository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ManifestHomepage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<ManifestDocumentation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<ManifestExample>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ManifestCapability>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<ManifestDependency>,
    #[serde(default)]
    pub relationships: ManifestRelationshipSet,
    pub trust: ManifestTrust,
    #[serde(default)]
    pub health: ManifestHealth,
    #[serde(default)]
    pub telemetry: ManifestTelemetry,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ManifestProvenance>,
    #[serde(default)]
    pub lifecycle: ManifestLifecycle,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<ManifestSignature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<ManifestChecksum>,
    #[serde(default)]
    pub compatibility: ManifestCompatibility,
    #[serde(default)]
    pub extensions: ManifestExtensions,
    #[serde(default)]
    pub metadata: ManifestMetadata,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl ConstitutionalManifest {
    pub fn new(
        name: impl Into<String>,
        kind: ManifestKind,
        version: ManifestVersion,
        description: impl Into<String>,
    ) -> Self {
        ConstitutionalManifest {
            identity: IdentityManifest::new(name, kind, version),
            schema_version: ManifestSchemaVersion::default(),
            description: description.into(),
            author: None,
            license: None,
            repository: None,
            homepage: None,
            documentation: None,
            examples: Vec::new(),
            capabilities: Vec::new(),
            dependencies: Vec::new(),
            relationships: ManifestRelationshipSet::default(),
            trust: ManifestTrust::new(TrustLevel::Unknown),
            health: ManifestHealth::default(),
            telemetry: ManifestTelemetry::default(),
            provenance: None,
            lifecycle: ManifestLifecycle::default(),
            signature: None,
            checksum: None,
            compatibility: ManifestCompatibility::default(),
            extensions: ManifestExtensions::default(),
            metadata: ManifestMetadata::default(),
            tags: Vec::new(),
            category: None,
        }
    }

    pub fn id(&self) -> String {
        self.identity.id()
    }
}

/// Builder for . Provides a
/// fluent API for constructing manifests without
/// having to fill every field at once.
pub struct ConstitutionalManifestBuilder {
    inner: ConstitutionalManifest,
}

impl ConstitutionalManifestBuilder {
    pub fn new(name: impl Into<String>, kind: ManifestKind, version: ManifestVersion) -> Self {
        ConstitutionalManifestBuilder {
            inner: ConstitutionalManifest {
                identity: IdentityManifest::new(name, kind, version),
                schema_version: ManifestSchemaVersion::default(),
                description: String::new(),
                author: None,
                license: None,
                repository: None,
                homepage: None,
                documentation: None,
                examples: Vec::new(),
                capabilities: Vec::new(),
                dependencies: Vec::new(),
                relationships: ManifestRelationshipSet::default(),
                trust: ManifestTrust::new(TrustLevel::Unknown),
                health: ManifestHealth::default(),
                telemetry: ManifestTelemetry::default(),
                provenance: None,
                lifecycle: ManifestLifecycle::default(),
                signature: None,
                checksum: None,
                compatibility: ManifestCompatibility::default(),
                extensions: ManifestExtensions::default(),
                metadata: ManifestMetadata::default(),
                tags: Vec::new(),
                category: None,
            },
        }
    }

    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.inner.description = d.into();
        self
    }

    pub fn author(mut self, a: ManifestAuthor) -> Self {
        self.inner.author = Some(a);
        self
    }

    pub fn license(mut self, l: ManifestLicense) -> Self {
        self.inner.license = Some(l);
        self
    }

    pub fn repository(mut self, r: ManifestRepository) -> Self {
        self.inner.repository = Some(r);
        self
    }

    pub fn homepage(mut self, url: impl Into<String>) -> Self {
        self.inner.homepage = Some(ManifestHomepage { url: url.into() });
        self
    }

    pub fn documentation(mut self, url: impl Into<String>) -> Self {
        self.inner.documentation = Some(ManifestDocumentation { url: url.into() });
        self
    }

    pub fn add_capability(mut self, c: ManifestCapability) -> Self {
        self.inner.capabilities.push(c);
        self
    }

    pub fn add_dependency(mut self, d: ManifestDependency) -> Self {
        self.inner.dependencies.push(d);
        self
    }

    pub fn add_relationship(mut self, r: ManifestRelationship) -> Self {
        self.inner.relationships.relationships.push(r);
        self
    }

    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.inner.tags.push(tag.into());
        self
    }

    pub fn category(mut self, c: impl Into<String>) -> Self {
        self.inner.category = Some(c.into());
        self
    }

    pub fn trust(mut self, t: ManifestTrust) -> Self {
        self.inner.trust = t;
        self
    }

    pub fn signature(mut self, s: ManifestSignature) -> Self {
        self.inner.signature = Some(s);
        self
    }

    pub fn checksum(mut self, c: ManifestChecksum) -> Self {
        self.inner.checksum = Some(c);
        self
    }

    pub fn extension(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner
            .extensions
            .values
            .insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> ConstitutionalManifest {
        self.inner
    }
}

/// Validation errors returned by .
/// The validator checks the canonical fields of a
/// . It does not interpret
///  or ; those
/// are constitutional-object-specific.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ManifestValidationError {
    #[error("manifest name must not be empty")]
    EmptyName,
    #[error("manifest description must not be empty")]
    EmptyDescription,
    #[error("manifest version is invalid (major must be > 0)")]
    InvalidVersion,
    #[error("manifest schema version is unsupported (expected 1.x, got {0})")]
    UnsupportedSchemaVersion(u32),
    #[error("dependency references unknown kind")]
    InvalidDependencyKind,
    #[error("duplicate dependency name")]
    DuplicateDependency,
    #[error("manifest has no capabilities declared")]
    NoCapabilities,
    #[error("declared capability name must not be empty")]
    EmptyCapabilityName,
}

/// The validator for .
pub struct ManifestValidator;

impl ManifestValidator {
    pub fn new() -> Self {
        ManifestValidator
    }

    /// Validate a manifest. Returns the first
    /// validation error or  if valid.
    pub fn validate(&self, m: &ConstitutionalManifest) -> Result<(), ManifestValidationError> {
        if m.identity.name.trim().is_empty() {
            return Err(ManifestValidationError::EmptyName);
        }
        if m.description.trim().is_empty() {
            return Err(ManifestValidationError::EmptyDescription);
        }
        if m.identity.version.major == 0 {
            return Err(ManifestValidationError::InvalidVersion);
        }
        if m.schema_version.major != 1 {
            return Err(ManifestValidationError::UnsupportedSchemaVersion(
                m.schema_version.major,
            ));
        }
        // Validate capabilities.
        for cap in &m.capabilities {
            if cap.name.trim().is_empty() {
                return Err(ManifestValidationError::EmptyCapabilityName);
            }
        }
        // Validate dependencies.
        let mut seen = std::collections::BTreeSet::new();
        for dep in &m.dependencies {
            if !seen.insert(dep.name.clone()) {
                return Err(ManifestValidationError::DuplicateDependency);
            }
        }
        Ok(())
    }
}

impl Default for ManifestValidator {
    fn default() -> Self {
        ManifestValidator::new()
    }
}

/// A trait for serializing manifests to a string
/// representation. Implementations: ,
/// , .
pub trait ManifestSerializer: Send + Sync {
    /// The format name (e.g. "json", "yaml", "toml").
    fn format_name(&self) -> &str;

    /// Serialize a manifest to a string. Returns an
    /// error string if the manifest is invalid for
    /// this format.
    fn serialize(&self, m: &ConstitutionalManifest) -> Result<String, String>;
}

/// A trait for deserializing manifests.
pub trait ManifestDeserializer: Send + Sync {
    fn format_name(&self) -> &str;
    fn deserialize(&self, s: &str) -> Result<ConstitutionalManifest, String>;
}

/// JSON serializer. The default implementation uses
/// .
pub struct JsonManifestSerializer;

impl JsonManifestSerializer {
    pub fn new() -> Self {
        JsonManifestSerializer
    }
}

impl Default for JsonManifestSerializer {
    fn default() -> Self {
        JsonManifestSerializer::new()
    }
}

impl ManifestSerializer for JsonManifestSerializer {
    fn format_name(&self) -> &str {
        "json"
    }
    fn serialize(&self, m: &ConstitutionalManifest) -> Result<String, String> {
        serde_json::to_string_pretty(m).map_err(|e| e.to_string())
    }
}

impl ManifestDeserializer for JsonManifestSerializer {
    fn format_name(&self) -> &str {
        "json"
    }
    fn deserialize(&self, s: &str) -> Result<ConstitutionalManifest, String> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}

/// YAML serializer. The runtime uses the
/// crate when this is enabled. The default constructor
/// returns an error (YAML is not always available).
/// KUBER Palace installs a real implementation.
pub struct YamlManifestSerializer;

impl Default for YamlManifestSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl YamlManifestSerializer {
    pub fn new() -> Self {
        YamlManifestSerializer
    }
}

impl ManifestSerializer for YamlManifestSerializer {
    fn format_name(&self) -> &str {
        "yaml"
    }
    fn serialize(&self, _m: &ConstitutionalManifest) -> Result<String, String> {
        Err("YAML support requires the serde_yaml feature".to_string())
    }
}

impl ManifestDeserializer for YamlManifestSerializer {
    fn format_name(&self) -> &str {
        "yaml"
    }
    fn deserialize(&self, _s: &str) -> Result<ConstitutionalManifest, String> {
        Err("YAML support requires the serde_yaml feature".to_string())
    }
}

/// TOML serializer. Same pattern as YAML: requires an
/// optional dependency.
pub struct TomlManifestSerializer;

impl Default for TomlManifestSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlManifestSerializer {
    pub fn new() -> Self {
        TomlManifestSerializer
    }
}

impl ManifestSerializer for TomlManifestSerializer {
    fn format_name(&self) -> &str {
        "toml"
    }
    fn serialize(&self, _m: &ConstitutionalManifest) -> Result<String, String> {
        Err("TOML support requires the toml feature".to_string())
    }
}

impl ManifestDeserializer for TomlManifestSerializer {
    fn format_name(&self) -> &str {
        "toml"
    }
    fn deserialize(&self, _s: &str) -> Result<ConstitutionalManifest, String> {
        Err("TOML support requires the toml feature".to_string())
    }
}

/// A trait for loading manifests. Loaders are pluggable:
/// file-based, network-based, in-memory. The runtime
/// uses this to populate its registries at startup.
pub trait ManifestLoader: Send + Sync {
    fn loader_name(&self) -> &str;
    fn load(&self) -> Vec<ConstitutionalManifest>;
}

/// An in-memory loader for tests and embedded use.
pub struct InMemoryManifestLoader {
    name: String,
    manifests: Vec<ConstitutionalManifest>,
}

impl InMemoryManifestLoader {
    pub fn new(name: impl Into<String>) -> Self {
        InMemoryManifestLoader {
            name: name.into(),
            manifests: Vec::new(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, m: ConstitutionalManifest) -> Self {
        self.manifests.push(m);
        self
    }
}

impl ManifestLoader for InMemoryManifestLoader {
    fn loader_name(&self) -> &str {
        &self.name
    }
    fn load(&self) -> Vec<ConstitutionalManifest> {
        self.manifests.clone()
    }
}

/// A thread-safe registry of s.
/// The runtime uses this to look up manifests by
/// identity, kind, version, tag, or category.
pub struct ManifestRegistry {
    inner: std::sync::RwLock<Vec<ConstitutionalManifest>>,
}

impl ManifestRegistry {
    pub fn new() -> Self {
        ManifestRegistry {
            inner: std::sync::RwLock::new(Vec::new()),
        }
    }

    pub fn register(&self, m: ConstitutionalManifest) {
        let mut guard = self.inner.write().expect("registry poisoned");
        guard.push(m);
    }

    pub fn unregister(&self, name: &str) -> bool {
        let mut guard = self.inner.write().expect("registry poisoned");
        let before = guard.len();
        guard.retain(|m| m.identity.name != name);
        guard.len() != before
    }

    pub fn lookup(&self, name: &str) -> Option<ConstitutionalManifest> {
        let guard = self.inner.read().expect("registry poisoned");
        guard.iter().find(|m| m.identity.name == name).cloned()
    }

    pub fn lookup_by_identity(
        &self,
        identity: &IdentityManifest,
    ) -> Option<ConstitutionalManifest> {
        let guard = self.inner.read().expect("registry poisoned");
        guard
            .iter()
            .find(|m| m.identity.name == identity.name && m.identity.version == identity.version)
            .cloned()
    }

    pub fn lookup_by_kind(&self, kind: &ManifestKind) -> Vec<ConstitutionalManifest> {
        let guard = self.inner.read().expect("registry poisoned");
        guard
            .iter()
            .filter(|m| &m.identity.kind == kind)
            .cloned()
            .collect()
    }

    pub fn lookup_by_version(
        &self,
        name: &str,
        version: &ManifestVersion,
    ) -> Option<ConstitutionalManifest> {
        let guard = self.inner.read().expect("registry poisoned");
        guard
            .iter()
            .find(|m| m.identity.name == name && &m.identity.version == version)
            .cloned()
    }

    pub fn lookup_by_tag(&self, tag: &str) -> Vec<ConstitutionalManifest> {
        let guard = self.inner.read().expect("registry poisoned");
        guard
            .iter()
            .filter(|m| m.tags.iter().any(|t| t == tag))
            .cloned()
            .collect()
    }

    pub fn lookup_by_category(&self, category: &str) -> Vec<ConstitutionalManifest> {
        let guard = self.inner.read().expect("registry poisoned");
        guard
            .iter()
            .filter(|m| m.category.as_deref() == Some(category))
            .cloned()
            .collect()
    }

    pub fn validate(&self) -> Vec<(String, ManifestValidationError)> {
        let guard = self.inner.read().expect("registry poisoned");
        let validator = ManifestValidator::new();
        let mut errors = Vec::new();
        for m in guard.iter() {
            if let Err(e) = validator.validate(m) {
                errors.push((m.identity.name.clone(), e));
            }
        }
        errors
    }

    pub fn list(&self) -> Vec<ConstitutionalManifest> {
        let guard = self.inner.read().expect("registry poisoned");
        guard.clone()
    }

    pub fn len(&self) -> usize {
        let guard = self.inner.read().expect("registry poisoned");
        guard.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for ManifestRegistry {
    fn default() -> Self {
        ManifestRegistry::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> ConstitutionalManifest {
        ConstitutionalManifestBuilder::new(
            "phoenix",
            ManifestKind::SourceHarness,
            ManifestVersion::new(1, 0, 0),
        )
        .description("Execution source harness")
        .author(ManifestAuthor::new("Arka"))
        .license(ManifestLicense::spdx("MIT"))
        .repository(ManifestRepository::git("https://example.com/phoenix"))
        .homepage("https://example.com/phoenix")
        .documentation("https://docs.example.com/phoenix")
        .add_capability(ManifestCapability::new("execution", "runs code"))
        .add_capability(ManifestCapability::new("sandbox", "isolated execution"))
        .add_dependency(
            ManifestDependency::new("anubis", ManifestKind::SourceHarness)
                .with_version(ManifestVersion::new(1, 0, 0)),
        )
        .add_relationship(ManifestRelationship::new("depends-on", "anubis"))
        .add_tag("execution")
        .add_tag("core")
        .category("execution-harness")
        .trust(ManifestTrust::new(TrustLevel::Official).verified())
        .signature(ManifestSignature::new("ed25519", "abc123", "key-001"))
        .checksum(ManifestChecksum::new("sha256", "deadbeef"))
        .build()
    }

    #[test]
    fn manifest_identity_id() {
        let m = fixture();
        assert_eq!(m.id(), "phoenix@1.0.0");
    }

    #[test]
    fn builder_produces_valid_manifest() {
        let m = fixture();
        assert_eq!(m.identity.name, "phoenix");
        assert_eq!(m.identity.version, ManifestVersion::new(1, 0, 0));
        assert_eq!(m.capabilities.len(), 2);
        assert_eq!(m.dependencies.len(), 1);
        assert_eq!(m.tags, vec!["execution", "core"]);
    }

    #[test]
    fn manifest_version_string() {
        let v = ManifestVersion::new(1, 2, 3);
        assert_eq!(v.as_string(), "1.2.3");
    }

    #[test]
    fn manifest_kind_names() {
        assert_eq!(ManifestKind::SourceHarness.name(), "SourceHarness");
        assert_eq!(ManifestKind::Mcp.name(), "MCP");
        assert_eq!(ManifestKind::Custom("x".to_string()).name(), "x");
    }

    #[test]
    fn manifest_serializes_to_json_and_back() {
        let m = fixture();
        let s = JsonManifestSerializer::new().serialize(&m).unwrap();
        let m2 = JsonManifestSerializer::new().deserialize(&s).unwrap();
        assert_eq!(m, m2);
    }

    #[test]
    fn validator_accepts_valid() {
        let m = fixture();
        assert!(ManifestValidator::new().validate(&m).is_ok());
    }

    #[test]
    fn validator_rejects_empty_name() {
        let mut m = fixture();
        m.identity.name = "   ".to_string();
        assert_eq!(
            ManifestValidator::new().validate(&m),
            Err(ManifestValidationError::EmptyName)
        );
    }

    #[test]
    fn validator_rejects_zero_major_version() {
        let mut m = fixture();
        m.identity.version = ManifestVersion::new(0, 1, 0);
        assert_eq!(
            ManifestValidator::new().validate(&m),
            Err(ManifestValidationError::InvalidVersion)
        );
    }

    #[test]
    fn validator_rejects_unsupported_schema_major() {
        let mut m = fixture();
        m.schema_version = ManifestSchemaVersion::new(2, 0);
        assert_eq!(
            ManifestValidator::new().validate(&m),
            Err(ManifestValidationError::UnsupportedSchemaVersion(2))
        );
    }

    #[test]
    fn registry_register_and_lookup() {
        let r = ManifestRegistry::new();
        let m = fixture();
        r.register(m.clone());
        let found = r.lookup("phoenix").unwrap();
        assert_eq!(m, found);
    }

    #[test]
    fn registry_lookup_by_identity() {
        let r = ManifestRegistry::new();
        r.register(fixture());
        let found = r
            .lookup_by_identity(&IdentityManifest::new(
                "phoenix",
                ManifestKind::SourceHarness,
                ManifestVersion::new(1, 0, 0),
            ))
            .unwrap();
        assert_eq!(found.id(), "phoenix@1.0.0");
    }

    #[test]
    fn registry_lookup_by_kind() {
        let r = ManifestRegistry::new();
        r.register(fixture());
        let results = r.lookup_by_kind(&ManifestKind::SourceHarness);
        assert_eq!(results.len(), 1);
        let results = r.lookup_by_kind(&ManifestKind::Agent);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn registry_lookup_by_tag() {
        let r = ManifestRegistry::new();
        r.register(fixture());
        let results = r.lookup_by_tag("execution");
        assert_eq!(results.len(), 1);
        let results = r.lookup_by_tag("missing");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn registry_lookup_by_category() {
        let r = ManifestRegistry::new();
        r.register(fixture());
        let results = r.lookup_by_category("execution-harness");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn registry_unregister() {
        let r = ManifestRegistry::new();
        r.register(fixture());
        assert_eq!(r.len(), 1);
        assert!(r.unregister("phoenix"));
        assert_eq!(r.len(), 0);
        assert!(!r.unregister("phoenix"));
    }

    #[test]
    fn registry_validate() {
        let r = ManifestRegistry::new();
        r.register(fixture());
        let errors = r.validate();
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn in_memory_loader_returns_added() {
        let l = InMemoryManifestLoader::new("test").add(fixture());
        assert_eq!(l.loader_name(), "test");
        let loaded = l.load();
        assert_eq!(loaded.len(), 1);
    }

    #[test]
    fn compatibility_builder() {
        let c = ManifestCompatibility::new()
            .upgrade_from(ManifestVersion::new(0, 9, 0))
            .with_migration_note("see changelog");
        assert_eq!(c.upgrade_from, Some(ManifestVersion::new(0, 9, 0)));
        assert_eq!(c.migration_notes.len(), 1);
    }

    #[test]
    fn relationship_set() {
        let r = ManifestRelationshipSet::new()
            .add(ManifestRelationship::new("extends", "base"))
            .add(ManifestRelationship::new("replaces", "old"));
        assert_eq!(r.relationships.len(), 2);
    }

    #[test]
    fn extensions_set_get() {
        let e = ManifestExtensions::new().set("k", "v").set("k2", "v2");
        assert_eq!(e.get("k"), Some("v"));
        assert_eq!(e.get("k2"), Some("v2"));
        assert_eq!(e.get("missing"), None);
    }

    #[test]
    fn trust_levels() {
        let t = ManifestTrust::new(TrustLevel::Official).verified();
        assert!(t.verified);
        assert_eq!(t.level, TrustLevel::Official);
    }

    #[test]
    fn lifecycle_state_default() {
        let l = ManifestLifecycle::default();
        assert_eq!(l.state, ManifestLifecycleState::Registered);
    }

    #[test]
    fn yaml_serializer_reports_unavailable() {
        let m = fixture();
        let r = YamlManifestSerializer::new().serialize(&m);
        assert!(r.is_err());
    }

    #[test]
    fn toml_serializer_reports_unavailable() {
        let m = fixture();
        let r = TomlManifestSerializer::new().serialize(&m);
        assert!(r.is_err());
    }

    #[test]
    fn future_kind_compiles() {
        // Future constitutional objects can use the
        // Custom variant without modifying the enum.
        let m = ConstitutionalManifestBuilder::new(
            "my-future-thing",
            ManifestKind::Custom("FutureThing".to_string()),
            ManifestVersion::new(0, 1, 0),
        )
        .description("future")
        .build();
        assert_eq!(m.identity.kind.name(), "FutureThing");
    }
}
