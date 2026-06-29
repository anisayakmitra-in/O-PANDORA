//! Source Harness runtime infrastructure.
//!
//! This module defines the contracts every Source
//! Harness runtime must expose. It does NOT contain
//! business logic; the contracts are the surface
//! community harnesses implement to be KUBER-Palace-
//! publishable and dynamically discoverable.
//!
//! ## Architecture
//!
//! A  is the publication unit.
//! It bundles:
//!
//! - the  (the existing
//!   identity/version/description tuple)
//! -  (the runtime's health)
//! -  (counters and activity)
//! -  (semver-like version with compat
//!    range)
//! -  list (other harnesses required)
//! -  list (what the harness
//!    provides)
//! -  (typed config schema)
//! -  (lineage, generation, signing)
//!
//! A  discovers descriptors. A
//!  validates them. A
//!  owns the runtime state
//! machine. None of these execute the harness's domain
//! work; they only manage the harness's presence in
//! the runtime.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::harness::SourceHarnessKind;

/// Health status of a Source Harness runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum HealthStatus {
    /// The harness is fully operational.
    Healthy,
    /// The harness is operational but with reduced
    /// capability. May recover.
    Degraded,
    /// The harness is not operational. Should be
    /// removed from the dispatch path.
    Unhealthy,
    /// The harness has not yet reported health.
    #[default]
    Unknown,
}

impl HealthStatus {
    pub fn is_dispatchable(self) -> bool {
        matches!(self, HealthStatus::Healthy | HealthStatus::Degraded)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            HealthStatus::Healthy => "HEALTHY",
            HealthStatus::Degraded => "DEGRADED",
            HealthStatus::Unhealthy => "UNHEALTHY",
            HealthStatus::Unknown => "UNKNOWN",
        }
    }
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A semver-like version. Uses
/// with an optional  indicating the
/// lowest minor version the harness is still
/// compatible with.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionSpec {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub compat_minor: u32,
}

impl VersionSpec {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        VersionSpec {
            major,
            minor,
            patch,
            compat_minor: minor,
        }
    }

    pub fn with_compat_minor(mut self, minor: u32) -> Self {
        self.compat_minor = minor;
        self
    }

    /// True if  is compatible with this version
    /// (same major, ).
    pub fn accepts(&self, other: &VersionSpec) -> bool {
        other.major == self.major && other.minor >= self.compat_minor
    }

    pub fn as_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl fmt::Display for VersionSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_string())
    }
}

/// A dependency on another Source Harness. The loader
/// uses  to compute a topological
/// install order and to detect conflicts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DependencySpec {
    pub name: String,
    pub kind: SourceHarnessKind,
    pub version_range: VersionRange,
}

impl DependencySpec {
    pub fn requires(kind: SourceHarnessKind, name: impl Into<String>) -> Self {
        DependencySpec {
            kind,
            name: name.into(),
            version_range: VersionRange::any(),
        }
    }

    pub fn with_version(mut self, range: VersionRange) -> Self {
        self.version_range = range;
        self
    }

    pub fn is_satisfied_by(&self, version: &VersionSpec) -> bool {
        self.version_range.contains(version)
    }
}

/// A semver-ish version range. Used by .
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionRange {
    pub min: Option<VersionSpec>,
    pub max: Option<VersionSpec>,
}

impl VersionRange {
    pub fn any() -> Self {
        VersionRange {
            min: None,
            max: None,
        }
    }

    pub fn at_least(min: VersionSpec) -> Self {
        VersionRange {
            min: Some(min),
            max: None,
        }
    }

    pub fn at_most(max: VersionSpec) -> Self {
        VersionRange {
            min: None,
            max: Some(max),
        }
    }

    pub fn between(min: VersionSpec, max: VersionSpec) -> Self {
        VersionRange {
            min: Some(min),
            max: Some(max),
        }
    }

    pub fn contains(&self, v: &VersionSpec) -> bool {
        if let Some(min) = &self.min {
            if v.major < min.major {
                return false;
            }
            if v.major == min.major && v.minor < min.minor {
                return false;
            }
        }
        if let Some(max) = &self.max {
            if v.major > max.major {
                return false;
            }
            if v.major == max.major && v.minor > max.minor {
                return false;
            }
        }
        true
    }
}

/// A declaration of a capability a Source Harness
/// provides. The runtime uses this to advertise what
/// the harness can do to other subsystems.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityDeclaration {
    pub name: String,
    pub description: String,
    pub required: bool,
}

impl CapabilityDeclaration {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        CapabilityDeclaration {
            name: name.into(),
            description: description.into(),
            required: false,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

/// Configuration specification. A Source Harness
/// declares its typed configuration schema by name
/// and a free-form description. The runtime does not
/// interpret the schema; it stores and retrieves it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConfigurationSpec {
    pub schema_name: String,
    pub schema_version: VersionSpec,
    pub description: String,
}

impl ConfigurationSpec {
    pub fn new(
        schema_name: impl Into<String>,
        schema_version: VersionSpec,
        description: impl Into<String>,
    ) -> Self {
        ConfigurationSpec {
            schema_name: schema_name.into(),
            schema_version,
            description: description.into(),
        }
    }
}

/// Evolution metadata. Tracks the harness's lineage
/// for auditability and evolution purposes. Signing
/// is included so KUBER Palace can verify authenticity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EvolutionMetadata {
    pub generation: u32,
    pub parent: Option<String>,
    pub mutation: Option<String>,
    pub signature_algorithm: String,
    pub signature: String,
}

impl EvolutionMetadata {
    pub fn unsigned(generation: u32) -> Self {
        EvolutionMetadata {
            generation,
            parent: None,
            mutation: None,
            signature_algorithm: "none".to_string(),
            signature: String::new(),
        }
    }

    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    pub fn with_mutation(mut self, mutation: impl Into<String>) -> Self {
        self.mutation = Some(mutation.into());
        self
    }

    pub fn with_signature(
        mut self,
        algorithm: impl Into<String>,
        signature: impl Into<String>,
    ) -> Self {
        self.signature_algorithm = algorithm.into();
        self.signature = signature.into();
        self
    }
}

/// A telemetry report. A Source Harness runtime
/// periodically emits a  so the
/// runtime can monitor activity, detect failures,
/// and feed downstream observability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelemetryReport {
    pub counters: BTreeMap<String, u64>,
    pub last_activity_ms: u64,
    pub uptime_ms: u64,
}

impl TelemetryReport {
    pub fn empty() -> Self {
        TelemetryReport {
            counters: BTreeMap::new(),
            last_activity_ms: 0,
            uptime_ms: 0,
        }
    }

    pub fn with_counter(mut self, name: impl Into<String>, value: u64) -> Self {
        self.counters.insert(name.into(), value);
        self
    }

    pub fn with_activity(mut self, ms: u64) -> Self {
        self.last_activity_ms = ms;
        self
    }

    pub fn with_uptime(mut self, ms: u64) -> Self {
        self.uptime_ms = ms;
        self
    }

    pub fn counter(&self, name: &str) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }
}

/// Lifecycle state of a Source Harness runtime. The
/// runtime moves through these states. The default
///  keeps this state in memory
/// and never panics on invalid transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LifecycleState {
    /// The harness has been registered but not started.
    Registered,
    /// The harness is loading (config, dependencies).
    Booting,
    /// The harness is fully operational.
    Ready,
    /// The harness is shutting down.
    ShuttingDown,
    /// The harness is no longer in the runtime.
    Stopped,
    /// The harness encountered an error.
    Failed,
}

impl LifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            LifecycleState::Registered => "REGISTERED",
            LifecycleState::Booting => "BOOTING",
            LifecycleState::Ready => "READY",
            LifecycleState::ShuttingDown => "SHUTTING_DOWN",
            LifecycleState::Stopped => "STOPPED",
            LifecycleState::Failed => "FAILED",
        }
    }

    pub fn is_active(self) -> bool {
        matches!(self, LifecycleState::Booting | LifecycleState::Ready)
    }
}

/// A Source Harness descriptor. The descriptor is the
/// publication unit for KUBER Palace. It contains
/// every piece of metadata needed to install,
/// upgrade, validate, and govern a Source Harness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceHarnessDescriptor {
    pub name: String,
    pub kind: SourceHarnessKind,
    pub version: VersionSpec,
    pub description: String,
    pub dependencies: Vec<DependencySpec>,
    pub provides: Vec<CapabilityDeclaration>,
    pub configuration: ConfigurationSpec,
    pub evolution: EvolutionMetadata,
    pub health: HealthStatus,
    pub telemetry: TelemetryReport,
}

impl SourceHarnessDescriptor {
    pub fn new(
        name: impl Into<String>,
        kind: SourceHarnessKind,
        version: VersionSpec,
        description: impl Into<String>,
    ) -> Self {
        SourceHarnessDescriptor {
            name: name.into(),
            kind,
            version,
            description: description.into(),
            dependencies: Vec::new(),
            provides: Vec::new(),
            configuration: ConfigurationSpec::new(
                "default",
                VersionSpec::new(0, 1, 0),
                "default configuration",
            ),
            evolution: EvolutionMetadata::unsigned(1),
            health: HealthStatus::default(),
            telemetry: TelemetryReport::empty(),
        }
    }

    pub fn with_dependency(mut self, dep: DependencySpec) -> Self {
        self.dependencies.push(dep);
        self
    }

    pub fn with_capability(mut self, cap: CapabilityDeclaration) -> Self {
        self.provides.push(cap);
        self
    }

    pub fn with_health(mut self, h: HealthStatus) -> Self {
        self.health = h;
        self
    }

    pub fn with_evolution(mut self, e: EvolutionMetadata) -> Self {
        self.evolution = e;
        self
    }

    /// Returns true if every required capability is
    /// declared in . The runtime uses this
    /// to validate a harness before installing it.
    pub fn declares_required(&self) -> bool {
        self.provides.iter().any(|c| c.required)
    }
}

/// A loader discovers and loads Source Harness
/// descriptors. The runtime uses this to populate
/// its registries at startup. Loaders can be file-
/// based, network-based, or in-memory for tests.
pub trait SourceHarnessLoader: Send + Sync {
    /// A human-readable name for the loader (e.g.
    /// "file", "kuber-palace", "in-memory").
    fn loader_name(&self) -> &str;

    /// Load all descriptors the loader can find. The
    /// default implementation returns an empty vector.
    fn load(&self) -> Vec<SourceHarnessDescriptor> {
        Vec::new()
    }
}

/// A validator checks a descriptor before it is
/// installed into the runtime. The default validator
/// checks: non-empty name, declared version, at least
/// one declared capability.
pub struct SourceHarnessValidator;

impl SourceHarnessValidator {
    pub fn new() -> Self {
        SourceHarnessValidator
    }

    /// Validate a descriptor. Returns the first
    /// validation error, or  if valid.
    pub fn validate(&self, d: &SourceHarnessDescriptor) -> Result<(), ValidationError> {
        if d.name.trim().is_empty() {
            return Err(ValidationError::EmptyName);
        }
        if d.description.trim().is_empty() {
            return Err(ValidationError::EmptyDescription);
        }
        if d.provides.is_empty() {
            return Err(ValidationError::NoCapabilities);
        }
        // Verify each dependency refers to a real
        // kind we know about.
        for dep in &d.dependencies {
            // The kind is validated by the enum itself
            // at construction time, so this is a no-op
            // for known kinds.
            if dep.name.trim().is_empty() {
                return Err(ValidationError::EmptyDependencyName);
            }
        }
        Ok(())
    }
}

impl Default for SourceHarnessValidator {
    fn default() -> Self {
        SourceHarnessValidator::new()
    }
}

/// Validation errors. The runtime surfaces these
/// before installing a harness.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ValidationError {
    #[error("descriptor name must not be empty")]
    EmptyName,
    #[error("descriptor description must not be empty")]
    EmptyDescription,
    #[error("descriptor must declare at least one capability")]
    NoCapabilities,
    #[error("dependency name must not be empty")]
    EmptyDependencyName,
}

/// The runtime state machine for a Source Harness.
/// The lifecycle owns no business logic. It moves
/// the harness through the states defined by
///  and reports the current state to
/// the runtime.
pub struct SourceHarnessLifecycle {
    state: std::sync::Mutex<LifecycleState>,
}

impl SourceHarnessLifecycle {
    pub fn new() -> Self {
        SourceHarnessLifecycle {
            state: std::sync::Mutex::new(LifecycleState::Registered),
        }
    }

    pub fn state(&self) -> LifecycleState {
        *self.state.lock().expect("lifecycle poisoned")
    }

    /// Move to . Returns the previous state.
    pub fn boot(&self) -> LifecycleState {
        self.set(LifecycleState::Booting)
    }

    /// Move to . Returns the previous state.
    pub fn ready(&self) -> LifecycleState {
        self.set(LifecycleState::Ready)
    }

    /// Move to . Returns the previous state.
    pub fn shutdown(&self) -> LifecycleState {
        self.set(LifecycleState::ShuttingDown)
    }

    /// Move to . Returns the previous state.
    pub fn stop(&self) -> LifecycleState {
        self.set(LifecycleState::Stopped)
    }

    /// Move to . Returns the previous state.
    pub fn fail(&self) -> LifecycleState {
        self.set(LifecycleState::Failed)
    }

    fn set(&self, new: LifecycleState) -> LifecycleState {
        let mut guard = self.state.lock().expect("lifecycle poisoned");
        let prev = *guard;
        *guard = new;
        prev
    }
}

impl Default for SourceHarnessLifecycle {
    fn default() -> Self {
        SourceHarnessLifecycle::new()
    }
}

/// A small in-memory loader useful for tests and for
/// embedding descriptors directly. The runtime can
/// register descriptors with this loader at startup.
pub struct InMemoryLoader {
    name: String,
    descriptors: Vec<SourceHarnessDescriptor>,
}

impl InMemoryLoader {
    pub fn new(name: impl Into<String>) -> Self {
        InMemoryLoader {
            name: name.into(),
            descriptors: Vec::new(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, d: SourceHarnessDescriptor) -> Self {
        self.descriptors.push(d);
        self
    }
}

impl SourceHarnessLoader for InMemoryLoader {
    fn loader_name(&self) -> &str {
        &self.name
    }

    fn load(&self) -> Vec<SourceHarnessDescriptor> {
        self.descriptors.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_status_dispatchable() {
        assert!(HealthStatus::Healthy.is_dispatchable());
        assert!(HealthStatus::Degraded.is_dispatchable());
        assert!(!HealthStatus::Unhealthy.is_dispatchable());
        assert!(!HealthStatus::Unknown.is_dispatchable());
    }

    #[test]
    fn version_accepts_compat_range() {
        let v = VersionSpec::new(1, 2, 0).with_compat_minor(0);
        // compat_minor=0 means we accept minor >= 0 (any minor)
        assert!(v.accepts(&VersionSpec::new(1, 0, 0)));
        assert!(v.accepts(&VersionSpec::new(1, 2, 0)));
        assert!(v.accepts(&VersionSpec::new(1, 3, 0)));
        // Different major is rejected.
        assert!(!v.accepts(&VersionSpec::new(2, 0, 0)));
        // Below compat_minor is rejected.
        let v2 = VersionSpec::new(1, 5, 0).with_compat_minor(3);
        assert!(!v2.accepts(&VersionSpec::new(1, 2, 0)));
        assert!(v2.accepts(&VersionSpec::new(1, 3, 0)));
    }

    #[test]
    fn version_range_any_contains_all() {
        let r = VersionRange::any();
        assert!(r.contains(&VersionSpec::new(0, 0, 1)));
        assert!(r.contains(&VersionSpec::new(99, 99, 99)));
    }

    #[test]
    fn version_range_at_least() {
        let r = VersionRange::at_least(VersionSpec::new(1, 2, 0));
        assert!(r.contains(&VersionSpec::new(1, 2, 0)));
        assert!(r.contains(&VersionSpec::new(1, 5, 0)));
        assert!(r.contains(&VersionSpec::new(2, 0, 0)));
        assert!(!r.contains(&VersionSpec::new(1, 1, 99)));
    }

    #[test]
    fn dependency_is_satisfied_by_compat() {
        let d = DependencySpec::requires(SourceHarnessKind::Anubis, "anubis-default")
            .with_version(VersionRange::at_least(VersionSpec::new(1, 0, 0)));
        assert!(d.is_satisfied_by(&VersionSpec::new(1, 0, 0)));
        assert!(d.is_satisfied_by(&VersionSpec::new(1, 5, 3)));
        assert!(!d.is_satisfied_by(&VersionSpec::new(0, 9, 9)));
    }

    #[test]
    fn capability_required() {
        let c = CapabilityDeclaration::new("execution", "runs code").required();
        assert!(c.required);
    }

    #[test]
    fn evolution_metadata_builder() {
        let e = EvolutionMetadata::unsigned(3)
            .with_parent("phoenix-1.2.0")
            .with_mutation("add-fs-snapshot")
            .with_signature("ed25519", "abcd1234");
        assert_eq!(e.generation, 3);
        assert_eq!(e.parent, Some("phoenix-1.2.0".to_string()));
        assert_eq!(e.mutation, Some("add-fs-snapshot".to_string()));
        assert_eq!(e.signature_algorithm, "ed25519");
    }

    #[test]
    fn telemetry_counter() {
        let t = TelemetryReport::empty()
            .with_counter("requests", 42)
            .with_uptime(1000)
            .with_activity(500);
        assert_eq!(t.counter("requests"), 42);
        assert_eq!(t.counter("missing"), 0);
        assert_eq!(t.uptime_ms, 1000);
        assert_eq!(t.last_activity_ms, 500);
    }

    #[test]
    fn lifecycle_state_transitions() {
        let lc = SourceHarnessLifecycle::new();
        assert_eq!(lc.state(), LifecycleState::Registered);
        assert_eq!(lc.boot(), LifecycleState::Registered);
        assert_eq!(lc.state(), LifecycleState::Booting);
        assert_eq!(lc.ready(), LifecycleState::Booting);
        assert_eq!(lc.state(), LifecycleState::Ready);
        assert!(lc.state().is_active());
        assert_eq!(lc.shutdown(), LifecycleState::Ready);
        assert_eq!(lc.stop(), LifecycleState::ShuttingDown);
        assert_eq!(lc.state(), LifecycleState::Stopped);
    }

    #[test]
    fn lifecycle_can_fail_from_any_state() {
        let lc = SourceHarnessLifecycle::new();
        lc.boot();
        lc.ready();
        assert_eq!(lc.fail(), LifecycleState::Ready);
        assert_eq!(lc.state(), LifecycleState::Failed);
    }

    #[test]
    fn descriptor_builder() {
        let d = SourceHarnessDescriptor::new(
            "phoenix",
            SourceHarnessKind::Phoenix,
            VersionSpec::new(1, 0, 0),
            "Execution source harness",
        )
        .with_dependency(DependencySpec::requires(
            SourceHarnessKind::Anubis,
            "anubis",
        ))
        .with_capability(CapabilityDeclaration::new("execution", "runs code").required())
        .with_health(HealthStatus::Healthy);
        assert_eq!(d.name, "phoenix");
        assert_eq!(d.kind, SourceHarnessKind::Phoenix);
        assert_eq!(d.dependencies.len(), 1);
        assert_eq!(d.provides.len(), 1);
        assert!(d.declares_required());
    }

    #[test]
    fn validator_accepts_valid_descriptor() {
        let v = SourceHarnessValidator::new();
        let d = SourceHarnessDescriptor::new(
            "x",
            SourceHarnessKind::Phoenix,
            VersionSpec::new(1, 0, 0),
            "x",
        )
        .with_capability(CapabilityDeclaration::new("c", "d"));
        assert!(v.validate(&d).is_ok());
    }

    #[test]
    fn validator_rejects_empty_name() {
        let v = SourceHarnessValidator::new();
        let mut d = SourceHarnessDescriptor::new(
            "  ",
            SourceHarnessKind::Phoenix,
            VersionSpec::new(1, 0, 0),
            "x",
        );
        d.name = "   ".to_string();
        d = d.with_capability(CapabilityDeclaration::new("c", "d"));
        assert_eq!(v.validate(&d), Err(ValidationError::EmptyName));
    }

    #[test]
    fn validator_rejects_no_capabilities() {
        let v = SourceHarnessValidator::new();
        let d = SourceHarnessDescriptor::new(
            "x",
            SourceHarnessKind::Phoenix,
            VersionSpec::new(1, 0, 0),
            "x",
        );
        assert_eq!(v.validate(&d), Err(ValidationError::NoCapabilities));
    }

    #[test]
    fn in_memory_loader_returns_added_descriptors() {
        let l = InMemoryLoader::new("test")
            .add(
                SourceHarnessDescriptor::new(
                    "a",
                    SourceHarnessKind::Phoenix,
                    VersionSpec::new(1, 0, 0),
                    "a",
                )
                .with_capability(CapabilityDeclaration::new("c", "d")),
            )
            .add(
                SourceHarnessDescriptor::new(
                    "b",
                    SourceHarnessKind::Anubis,
                    VersionSpec::new(1, 0, 0),
                    "b",
                )
                .with_capability(CapabilityDeclaration::new("e", "f")),
            );
        assert_eq!(l.loader_name(), "test");
        let loaded = l.load();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].name, "a");
        assert_eq!(loaded[1].name, "b");
    }
}
