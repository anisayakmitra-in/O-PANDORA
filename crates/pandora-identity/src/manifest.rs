use std::collections::BTreeMap;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::kind::IdentityKind;
use crate::relationships::{Relationship, RelationshipKind, RelationshipSet};
use crate::version::IdentityVersion;

/// The status of a constitutional identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdentityStatus {
    Active,
    Disabled,
    Provisioning,
    Deprecated,
    Retired,
    Unavailable,
}

impl IdentityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IdentityStatus::Active => "active",
            IdentityStatus::Disabled => "disabled",
            IdentityStatus::Provisioning => "provisioning",
            IdentityStatus::Deprecated => "deprecated",
            IdentityStatus::Retired => "retired",
            IdentityStatus::Unavailable => "unavailable",
        }
    }

    pub fn is_usable(self) -> bool {
        matches!(self, IdentityStatus::Active | IdentityStatus::Provisioning)
    }
}

/// Lineage information.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityLineage {
    pub generation: u32,
    pub parent: Option<String>,
    pub evolved_from: Vec<String>,
}

impl IdentityLineage {
    pub fn new() -> Self {
        IdentityLineage::default()
    }
    pub fn with_generation(mut self, g: u32) -> Self {
        self.generation = g;
        self
    }
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }
}

/// A cryptographic signature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentitySignature {
    pub algorithm: String,
    pub value: String,
    pub key_id: Option<String>,
}

impl IdentitySignature {
    pub fn new(algorithm: impl Into<String>, value: impl Into<String>) -> Self {
        IdentitySignature {
            algorithm: algorithm.into(),
            value: value.into(),
            key_id: None,
        }
    }
    pub fn with_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.key_id = Some(key_id.into());
        self
    }
}

/// Health status.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum IdentityHealth {
    Healthy,
    Degraded,
    Unhealthy,
    #[default]
    Unknown,
}

impl IdentityHealth {
    pub fn as_str(self) -> &'static str {
        match self {
            IdentityHealth::Healthy => "healthy",
            IdentityHealth::Degraded => "degraded",
            IdentityHealth::Unhealthy => "unhealthy",
            IdentityHealth::Unknown => "unknown",
        }
    }
    pub fn is_acceptable(self) -> bool {
        matches!(self, IdentityHealth::Healthy | IdentityHealth::Degraded)
    }
}

pub type IdentityMetadata = BTreeMap<String, String>;

/// Capability declarations.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityCapabilities {
    pub provided: Vec<String>,
    pub required: Vec<String>,
}

impl IdentityCapabilities {
    pub fn new() -> Self {
        IdentityCapabilities::default()
    }
    pub fn provides(mut self, cap: impl Into<String>) -> Self {
        self.provided.push(cap.into());
        self
    }
    pub fn requires(mut self, cap: impl Into<String>) -> Self {
        self.required.push(cap.into());
        self
    }
}

/// Dependency declarations.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityDependencies {
    pub required: Vec<String>,
    pub optional: Vec<String>,
}

impl IdentityDependencies {
    pub fn new() -> Self {
        IdentityDependencies::default()
    }
    pub fn requires(mut self, id: impl Into<String>) -> Self {
        self.required.push(id.into());
        self
    }
    pub fn optionally_requires(mut self, id: impl Into<String>) -> Self {
        self.optional.push(id.into());
        self
    }
}

/// Trust metadata.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IdentityTrust {
    pub score: f32,
    pub verified_by: Vec<String>,
    pub notes: Vec<String>,
}

impl IdentityTrust {
    pub fn new() -> Self {
        IdentityTrust::default()
    }
}

/// Lifecycle metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum IdentityLifecycleStage {
    #[default]
    Declared,
    Installing,
    Installed,
    Upgrading,
    Uninstalling,
    Uninstalled,
}

/// Telemetry hook counts.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityTelemetry {
    pub emitted_kinds: Vec<String>,
}

impl IdentityTelemetry {
    pub fn new() -> Self {
        IdentityTelemetry::default()
    }
    pub fn emits(mut self, kind: impl Into<String>) -> Self {
        self.emitted_kinds.push(kind.into());
        self
    }
}

/// Provenance metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityProvenance {
    pub declared_by: Option<String>,
    pub approved_by: Vec<String>,
    pub published_by: Option<String>,
    pub source_repository: Option<String>,
    pub source_revision: Option<String>,
}

impl IdentityProvenance {
    pub fn new() -> Self {
        IdentityProvenance::default()
    }
    pub fn declared_by(mut self, who: impl Into<String>) -> Self {
        self.declared_by = Some(who.into());
        self
    }
    pub fn published_by(mut self, who: impl Into<String>) -> Self {
        self.published_by = Some(who.into());
        self
    }
}

/// The constitutional identity manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityManifest {
    pub id: String,
    pub name: String,
    pub kind: IdentityKind,
    pub version: IdentityVersion,
    pub author: String,
    pub lineage: IdentityLineage,
    pub dependencies: IdentityDependencies,
    pub capabilities: IdentityCapabilities,
    pub relationships: RelationshipSet,
    pub health: IdentityHealth,
    pub signature: Option<IdentitySignature>,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
    pub status: IdentityStatus,
    pub lifecycle: IdentityLifecycleStage,
    pub telemetry: IdentityTelemetry,
    pub trust: IdentityTrust,
    pub provenance: IdentityProvenance,
    pub metadata: IdentityMetadata,
    pub description: String,
}

impl IdentityManifest {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        kind: IdentityKind,
        author: impl Into<String>,
    ) -> Self {
        let now = SystemTime::now();
        IdentityManifest {
            id: id.into(),
            name: name.into(),
            kind,
            version: IdentityVersion::default(),
            author: author.into(),
            lineage: IdentityLineage::default(),
            dependencies: IdentityDependencies::default(),
            capabilities: IdentityCapabilities::default(),
            relationships: RelationshipSet::new(),
            health: IdentityHealth::default(),
            signature: None,
            created_at: now,
            modified_at: now,
            status: IdentityStatus::Active,
            lifecycle: IdentityLifecycleStage::default(),
            telemetry: IdentityTelemetry::default(),
            trust: IdentityTrust::default(),
            provenance: IdentityProvenance::default(),
            metadata: BTreeMap::new(),
            description: String::new(),
        }
    }

    pub fn with_version(mut self, v: IdentityVersion) -> Self {
        self.version = v;
        self
    }
    pub fn with_description(mut self, d: impl Into<String>) -> Self {
        self.description = d.into();
        self
    }
    pub fn with_status(mut self, s: IdentityStatus) -> Self {
        self.status = s;
        self
    }
    pub fn with_health(mut self, h: IdentityHealth) -> Self {
        self.health = h;
        self
    }
    pub fn with_lifecycle(mut self, l: IdentityLifecycleStage) -> Self {
        self.lifecycle = l;
        self
    }

    pub fn with_capability(mut self, cap: impl Into<String>) -> Self {
        self.capabilities = self.capabilities.provides(cap);
        self
    }
    pub fn requires_capability(mut self, cap: impl Into<String>) -> Self {
        self.capabilities = self.capabilities.requires(cap);
        self
    }
    pub fn depends_on(mut self, id: impl Into<String>) -> Self {
        self.dependencies = self.dependencies.requires(id);
        self
    }
    pub fn adds_relationship(mut self, kind: RelationshipKind, target: impl Into<String>) -> Self {
        self.relationships.add(Relationship::new(kind, target));
        self
    }
    pub fn with_signature(mut self, sig: IdentitySignature) -> Self {
        self.signature = Some(sig);
        self
    }
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    pub fn with_telemetry(mut self, kind: impl Into<String>) -> Self {
        self.telemetry = self.telemetry.emits(kind);
        self
    }

    pub fn is_usable(&self) -> bool {
        self.status.is_usable() && self.health.is_acceptable()
    }

    /// Return the canonical ConstitutionalManifest for this
    /// identity. This is the composition seam: the
    /// concrete `IdentityManifest` (which carries runtime
    /// fields like SystemTime, status, lineage) projects
    /// into the constitutional manifest shape. The
    /// constitutional manifest is the canonical owner of
    /// identity metadata in pandora-types.
    pub fn constitutional(&self) -> pandora_types::constitutional::ConstitutionalManifest {
        use pandora_types::constitutional::*;
        use std::collections::BTreeMap;
        use std::time::UNIX_EPOCH;

        let created_ms = self
            .created_at
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        let updated_ms = self
            .modified_at
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let provenance_str = format!(
            "declared_by={:?} approved_by={:?} published_by={:?} source_repo={:?} revision={:?}",
            self.provenance.declared_by,
            self.provenance.approved_by,
            self.provenance.published_by,
            self.provenance.source_repository,
            self.provenance.source_revision
        );

        let mut ext_values: BTreeMap<String, String> = BTreeMap::new();
        ext_values.insert("pandora_id".to_string(), self.id.clone());
        if !self.author.is_empty() {
            ext_values.insert("author".to_string(), self.author.clone());
        }
        ext_values.insert(
            "lineage_generation".to_string(),
            self.lineage.generation.to_string(),
        );
        if let Some(ref p) = self.lineage.parent {
            ext_values.insert("lineage_parent".to_string(), p.clone());
        }
        if !self.lineage.evolved_from.is_empty() {
            ext_values.insert(
                "lineage_evolved_from".to_string(),
                self.lineage.evolved_from.join(","),
            );
        }
        for (k, v) in &self.metadata {
            ext_values.insert(format!("meta.{}", k), v.clone());
        }
        let extensions = ManifestExtensions { values: ext_values };

        let mut meta_values: BTreeMap<String, String> = BTreeMap::new();
        meta_values.insert("created_ms".to_string(), created_ms.to_string());
        meta_values.insert("updated_ms".to_string(), updated_ms.to_string());
        meta_values.insert("status".to_string(), self.status.as_str().to_string());
        meta_values.insert("lifecycle".to_string(), format!("{:?}", self.lifecycle));
        let metadata = ManifestMetadata {
            values: meta_values,
        };

        let trust = ManifestTrust {
            level: trust_level_from(&self.trust),
            verified: self.trust.verified_by.iter().any(|v| !v.is_empty()),
        };

        let health = ManifestHealth {
            status: self.health.as_str().to_string(),
        };

        let mut counters: BTreeMap<String, u64> = BTreeMap::new();
        counters.insert(
            "emitted_kinds".to_string(),
            self.telemetry.emitted_kinds.len() as u64,
        );
        let telemetry = ManifestTelemetry { counters };

        let lifecycle = ManifestLifecycle {
            state: lifecycle_state_from(self.lifecycle),
        };

        let kind_manifest: ManifestKind = (&self.kind).into();
        let version_manifest: ManifestVersion = (&self.version).into();

        let mut manifest = ConstitutionalManifest::new(
            self.name.clone(),
            kind_manifest,
            version_manifest,
            self.description.clone(),
        );
        manifest.health = health;
        manifest.telemetry = telemetry;
        manifest.trust = trust;
        manifest.lifecycle = lifecycle;
        manifest.extensions = extensions;
        manifest.metadata = metadata;
        manifest.provenance = Some(ManifestProvenance::from_source(provenance_str));

        if let Some(ref sig) = self.signature {
            manifest.signature = Some(ManifestSignature::new(
                sig.algorithm.clone(),
                sig.value.clone(),
                sig.key_id.clone().unwrap_or_default(),
            ));
        }

        for cap in &self.capabilities.provided {
            manifest.capabilities.push(ManifestCapability::new(
                format!("provides.{}", cap),
                cap.clone(),
            ));
        }
        for cap in &self.capabilities.required {
            manifest.capabilities.push(ManifestCapability::new(
                format!("requires.{}", cap),
                cap.clone(),
            ));
        }
        for dep in &self.dependencies.required {
            manifest
                .dependencies
                .push(ManifestDependency::new(dep.clone(), ManifestKind::Package));
        }
        for dep in &self.dependencies.optional {
            manifest
                .dependencies
                .push(ManifestDependency::new(dep.clone(), ManifestKind::Plugin));
        }
        for r in self.relationships.iter() {
            manifest
                .relationships
                .relationships
                .push(ManifestRelationship::new(
                    format!("{:?}", r.kind),
                    r.target.clone(),
                ));
        }

        manifest
    }
}

fn trust_level_from(t: &IdentityTrust) -> pandora_types::constitutional::TrustLevel {
    if t.score >= 0.95 {
        pandora_types::constitutional::TrustLevel::Official
    } else if t.score >= 0.7 {
        pandora_types::constitutional::TrustLevel::Verified
    } else if t.score >= 0.3 {
        pandora_types::constitutional::TrustLevel::Community
    } else if t.score > 0.0 {
        pandora_types::constitutional::TrustLevel::Untrusted
    } else {
        pandora_types::constitutional::TrustLevel::Unknown
    }
}

fn lifecycle_state_from(
    s: IdentityLifecycleStage,
) -> pandora_types::constitutional::ManifestLifecycleState {
    use pandora_types::constitutional::ManifestLifecycleState;
    match s {
        IdentityLifecycleStage::Declared => ManifestLifecycleState::Registered,
        IdentityLifecycleStage::Installing => ManifestLifecycleState::Booting,
        IdentityLifecycleStage::Installed => ManifestLifecycleState::Ready,
        IdentityLifecycleStage::Upgrading => ManifestLifecycleState::Booting,
        IdentityLifecycleStage::Uninstalling => ManifestLifecycleState::ShuttingDown,
        IdentityLifecycleStage::Uninstalled => ManifestLifecycleState::Stopped,
    }
}

/// The constitutional identity trait.
pub trait Identity: Send + Sync + std::fmt::Debug {
    fn manifest(&self) -> &IdentityManifest;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_status_strings() {
        assert_eq!(IdentityStatus::Active.as_str(), "active");
        assert!(IdentityStatus::Active.is_usable());
        assert!(IdentityStatus::Provisioning.is_usable());
        assert!(!IdentityStatus::Disabled.is_usable());
    }

    #[test]
    fn identity_health_defaults_unknown() {
        let h = IdentityHealth::default();
        assert_eq!(h, IdentityHealth::Unknown);
        assert!(!h.is_acceptable());
    }

    #[test]
    fn identity_health_acceptable() {
        assert!(IdentityHealth::Healthy.is_acceptable());
        assert!(IdentityHealth::Degraded.is_acceptable());
        assert!(!IdentityHealth::Unhealthy.is_acceptable());
    }

    #[test]
    fn capabilities_builder() {
        let c = IdentityCapabilities::new()
            .provides("fs")
            .provides("shell")
            .requires("memory");
        assert_eq!(c.provided, vec!["fs", "shell"]);
        assert_eq!(c.required, vec!["memory"]);
    }

    #[test]
    fn dependencies_builder() {
        let d = IdentityDependencies::new()
            .requires("a")
            .optionally_requires("b");
        assert_eq!(d.required, vec!["a"]);
        assert_eq!(d.optional, vec!["b"]);
    }

    #[test]
    fn trust_default() {
        let t = IdentityTrust::new();
        assert_eq!(t.score, 0.0);
    }

    #[test]
    fn lineage_builder() {
        let l = IdentityLineage::new().with_generation(3).with_parent("v2");
        assert_eq!(l.generation, 3);
        assert_eq!(l.parent, Some("v2".to_string()));
    }

    #[test]
    fn signature_builder() {
        let s = IdentitySignature::new("ed25519", "abc").with_key_id("key-1");
        assert_eq!(s.algorithm, "ed25519");
        assert_eq!(s.key_id, Some("key-1".to_string()));
    }

    #[test]
    fn provenance_builder() {
        let p = IdentityProvenance::new()
            .declared_by("alice")
            .published_by("bob");
        assert_eq!(p.declared_by, Some("alice".to_string()));
    }

    #[test]
    fn telemetry_builder() {
        let t = IdentityTelemetry::new()
            .emits("execution.started")
            .emits("execution.finished");
        assert_eq!(t.emitted_kinds.len(), 2);
    }

    #[test]
    fn manifest_minimal() {
        let m = IdentityManifest::new(
            "pandora:phoenix",
            "phoenix",
            IdentityKind::SourceHarness,
            "kuber",
        );
        assert_eq!(m.id, "pandora:phoenix");
        assert_eq!(m.kind, IdentityKind::SourceHarness);
        assert_eq!(m.status, IdentityStatus::Active);
    }

    #[test]
    fn manifest_builder_chain() {
        let m = IdentityManifest::new(
            "pandora:phoenix.shell",
            "phoenix-shell",
            IdentityKind::MetaHarness,
            "kuber",
        )
        .with_description("Shell meta harness")
        .with_version(IdentityVersion::new(1, 0, 0))
        .with_capability("shell")
        .depends_on("pandora:phoenix")
        .adds_relationship(RelationshipKind::Parent, "pandora:phoenix")
        .with_signature(IdentitySignature::new("ed25519", "sig-1"))
        .with_metadata("license", "MIT")
        .with_telemetry("execution.started");
        assert_eq!(m.description, "Shell meta harness");
        assert_eq!(m.version.major, 1);
        assert_eq!(m.capabilities.provided.len(), 1);
        assert_eq!(m.dependencies.required, vec!["pandora:phoenix"]);
        assert_eq!(m.relationships.len(), 1);
        assert!(m.signature.is_some());
    }

    #[test]
    fn manifest_is_usable() {
        let mut m = IdentityManifest::new("x", "x", IdentityKind::Gene, "kuber");
        m.health = IdentityHealth::Healthy;
        assert!(m.is_usable());
        m.health = IdentityHealth::Unhealthy;
        assert!(!m.is_usable());
    }

    #[test]
    fn manifest_serializes() {
        let m = IdentityManifest::new("pandora:test", "test", IdentityKind::Tool, "kuber");
        let s = serde_json::to_string(&m).unwrap();
        let m2: IdentityManifest = serde_json::from_str(&s).unwrap();
        assert_eq!(m, m2);
    }

    #[test]
    fn constitutional_seam_identity_name() {
        let m = IdentityManifest::new(
            "pandora:phoenix",
            "phoenix",
            IdentityKind::SourceHarness,
            "kuber",
        )
        .with_description("Execution source harness");
        let c = m.constitutional();
        assert_eq!(c.identity.name, "phoenix");
        assert_eq!(
            c.identity.kind,
            pandora_types::constitutional::ManifestKind::SourceHarness
        );
    }

    #[test]
    fn constitutional_seam_carries_health_and_telemetry() {
        let mut m = IdentityManifest::new("pandora:x", "x", IdentityKind::Tool, "kuber");
        m.health = IdentityHealth::Healthy;
        m.telemetry = IdentityTelemetry::new().emits("exec.start");
        let c = m.constitutional();
        assert_eq!(c.health.status, "healthy");
        assert_eq!(c.telemetry.counters.get("emitted_kinds"), Some(&1));
    }

    #[test]
    fn constitutional_seam_round_trip_via_json() {
        let m = IdentityManifest::new(
            "pandora:phoenix.shell",
            "phoenix-shell",
            IdentityKind::MetaHarness,
            "kuber",
        )
        .with_description("Shell meta harness")
        .with_capability("shell")
        .with_health(IdentityHealth::Healthy);
        let c = m.constitutional();
        let json = serde_json::to_string(&c).unwrap();
        let c2: pandora_types::constitutional::ConstitutionalManifest =
            serde_json::from_str(&json).unwrap();
        assert_eq!(c, c2);
    }

    #[test]
    fn constitutional_seam_projects_capabilities_and_dependencies() {
        let m = IdentityManifest::new("pandora:y", "y", IdentityKind::Tool, "kuber")
            .with_capability("fs")
            .requires_capability("memory")
            .depends_on("pandora:anubis");
        let c = m.constitutional();
        let cap_names: Vec<&str> = c.capabilities.iter().map(|cap| cap.name.as_str()).collect();
        assert!(cap_names.contains(&"provides.fs"));
        assert!(cap_names.contains(&"requires.memory"));
        let dep_names: Vec<&str> = c.dependencies.iter().map(|d| d.name.as_str()).collect();
        assert!(dep_names.contains(&"pandora:anubis"));
    }
}
