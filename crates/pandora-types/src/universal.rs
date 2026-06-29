//! Universal constitutional models.
//!
//! These are the canonical implementations of:
//! - Health (universal health model)
//! - Lifecycle (universal lifecycle model)
//! - ExecutionProfile (execution classification)
//! - PandoraScore (publishable quality metadata)
//! - DebugPipeline (universal debug pipeline)
//!
//! Every constitutional object must use these models.
//! Do NOT redefine them. Compose or alias them.

use serde::{Deserialize, Serialize};

// ============================================================
// Universal Health Model
// ============================================================

/// The canonical health model for every constitutional
/// object. Every Source Harness, Meta Harness, Engine,
/// Gene, Workflow, Capability, and infrastructure object
/// uses this model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Health {
    #[default]
    Ready,
    Healthy,
    Busy,
    Degraded,
    Recovering,
    Repairing,
    Restarting,
    Quarantined,
    Offline,
    Archived,
}

impl Health {
    pub fn is_dispatchable(self) -> bool {
        matches!(self, Health::Ready | Health::Healthy | Health::Busy)
    }

    pub fn is_operational(self) -> bool {
        matches!(
            self,
            Health::Ready | Health::Healthy | Health::Busy | Health::Degraded | Health::Recovering
        )
    }
}

impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Health::Ready => write!(f, "ready"),
            Health::Healthy => write!(f, "healthy"),
            Health::Busy => write!(f, "busy"),
            Health::Degraded => write!(f, "degraded"),
            Health::Recovering => write!(f, "recovering"),
            Health::Repairing => write!(f, "repairing"),
            Health::Restarting => write!(f, "restarting"),
            Health::Quarantined => write!(f, "quarantined"),
            Health::Offline => write!(f, "offline"),
            Health::Archived => write!(f, "archived"),
        }
    }
}

// ============================================================
// Universal Lifecycle Model
// ============================================================

/// The canonical lifecycle model for every constitutional
/// object. Every Source Harness, Meta Harness, Gene,
/// Workflow, Capability, and infrastructure object uses
/// this lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Lifecycle {
    #[default]
    Created,
    Installed,
    Loaded,
    Initialized,
    Ready,
    Running,
    Paused,
    Recovering,
    Updating,
    Stopping,
    Stopped,
    Archived,
    Deprecated,
    Deleted,
}

impl Lifecycle {
    pub fn is_active(self) -> bool {
        matches!(
            self,
            Lifecycle::Ready | Lifecycle::Running | Lifecycle::Paused
        )
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Lifecycle::Archived | Lifecycle::Deprecated | Lifecycle::Deleted
        )
    }
}

impl std::fmt::Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lifecycle::Created => write!(f, "created"),
            Lifecycle::Installed => write!(f, "installed"),
            Lifecycle::Loaded => write!(f, "loaded"),
            Lifecycle::Initialized => write!(f, "initialized"),
            Lifecycle::Ready => write!(f, "ready"),
            Lifecycle::Running => write!(f, "running"),
            Lifecycle::Paused => write!(f, "paused"),
            Lifecycle::Recovering => write!(f, "recovering"),
            Lifecycle::Updating => write!(f, "updating"),
            Lifecycle::Stopping => write!(f, "stopping"),
            Lifecycle::Stopped => write!(f, "stopped"),
            Lifecycle::Archived => write!(f, "archived"),
            Lifecycle::Deprecated => write!(f, "deprecated"),
            Lifecycle::Deleted => write!(f, "deleted"),
        }
    }
}

// ============================================================
// Execution Profile
// ============================================================

/// Execution classification for constitutional objects.
/// Used by Phoenix to determine how to execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ExecutionProfile {
    #[default]
    Stateless,
    Stateful,
    Persistent,
    Distributed,
    Realtime,
    Offline,
    Experimental,
}

// ============================================================
// Universal Debug Pipeline
// ============================================================

/// Every Source Harness supports this debug pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DebugPhase {
    #[default]
    Trace,
    Diagnostics,
    Replay,
    Repair,
    Benchmark,
    Evolution,
    Publish,
}

// ============================================================
// GEPA / DSR Contracts
// ============================================================

/// GEPA/DSR configuration for constitutional objects.
/// Every object that supports evolution exposes this.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvolutionConfig {
    pub gepa_enabled: bool,
    pub dsr_enabled: bool,
}

impl EvolutionConfig {
    pub fn enabled() -> Self {
        EvolutionConfig {
            gepa_enabled: true,
            dsr_enabled: true,
        }
    }

    pub fn disabled() -> Self {
        EvolutionConfig {
            gepa_enabled: false,
            dsr_enabled: false,
        }
    }
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self::enabled()
    }
}

// ============================================================
// Pandora Score (metadata only)
// ============================================================

/// Quality metadata for published objects.
/// No business logic. Metadata only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PandoraScore {
    pub identity_present: bool,
    pub documentation_present: bool,
    pub tests_present: bool,
    pub telemetry_present: bool,
    pub health_present: bool,
    pub gepa_enabled: bool,
    pub dsr_enabled: bool,
    pub self_healing: bool,
    pub benchmark_coverage: bool,
    pub security_present: bool,
    pub trust_present: bool,
    pub governance_present: bool,
    pub compatibility_present: bool,
    pub official_status: bool,
}

impl PandoraScore {
    pub fn official() -> Self {
        PandoraScore {
            identity_present: true,
            documentation_present: true,
            tests_present: true,
            telemetry_present: true,
            health_present: true,
            gepa_enabled: true,
            dsr_enabled: true,
            self_healing: true,
            benchmark_coverage: true,
            security_present: true,
            trust_present: true,
            governance_present: true,
            compatibility_present: true,
            official_status: true,
        }
    }

    pub fn community() -> Self {
        PandoraScore {
            official_status: false,
            ..Self::official()
        }
    }
}

impl Default for PandoraScore {
    fn default() -> Self {
        Self::community()
    }
}

// ============================================================
// Workflow Constitutional Object
// ============================================================

/// A Workflow is a constitutional object that defines
/// an execution graph. It sits between Engine and
/// Capability in the hierarchy.
///
/// Architecture:
///   Source Harness → Meta Harness → Engine → Workflow → Capability → Gene
///
/// Workflows are publishable through KUBER Palace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub retry_policy: RetryPolicy,
    pub budget: WorkflowBudget,
}

/// A single step in a workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub capability: String,
    pub execution_profile: ExecutionProfile,
    pub timeout_ms: u64,
    pub retry_on_failure: bool,
}

/// Retry policy for workflow steps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub backoff_ms: u64,
    pub exponential: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        RetryPolicy {
            max_retries: 3,
            backoff_ms: 100,
            exponential: true,
        }
    }
}

/// Budget constraints for a workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowBudget {
    pub max_duration_ms: u64,
    pub max_cost_cents: u64,
    pub max_memory_mb: u64,
}

impl Default for WorkflowBudget {
    fn default() -> Self {
        WorkflowBudget {
            max_duration_ms: 60_000,
            max_cost_cents: 100,
            max_memory_mb: 1024,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_dispatchable() {
        assert!(Health::Ready.is_dispatchable());
        assert!(Health::Healthy.is_dispatchable());
        assert!(Health::Busy.is_dispatchable());
        assert!(!Health::Degraded.is_dispatchable());
        assert!(!Health::Offline.is_dispatchable());
    }

    #[test]
    fn health_operational() {
        assert!(Health::Ready.is_operational());
        assert!(Health::Degraded.is_operational());
        assert!(!Health::Quarantined.is_operational());
    }

    #[test]
    fn health_default() {
        assert_eq!(Health::default(), Health::Ready);
    }

    #[test]
    fn health_display() {
        assert_eq!(Health::Ready.to_string(), "ready");
        assert_eq!(Health::Quarantined.to_string(), "quarantined");
    }

    #[test]
    fn lifecycle_active() {
        assert!(Lifecycle::Ready.is_active());
        assert!(Lifecycle::Running.is_active());
        assert!(Lifecycle::Paused.is_active());
        assert!(!Lifecycle::Created.is_active());
    }

    #[test]
    fn lifecycle_terminal() {
        assert!(Lifecycle::Archived.is_terminal());
        assert!(Lifecycle::Deprecated.is_terminal());
        assert!(Lifecycle::Deleted.is_terminal());
        assert!(!Lifecycle::Ready.is_terminal());
    }

    #[test]
    fn lifecycle_default() {
        assert_eq!(Lifecycle::default(), Lifecycle::Created);
    }

    #[test]
    fn execution_profile_default() {
        assert_eq!(ExecutionProfile::default(), ExecutionProfile::Stateless);
    }

    #[test]
    fn evolution_config_enabled() {
        let c = EvolutionConfig::enabled();
        assert!(c.gepa_enabled);
        assert!(c.dsr_enabled);
    }

    #[test]
    fn evolution_config_disabled() {
        let c = EvolutionConfig::disabled();
        assert!(!c.gepa_enabled);
        assert!(!c.dsr_enabled);
    }

    #[test]
    fn pandora_score_official() {
        let s = PandoraScore::official();
        assert!(s.official_status);
        assert!(s.gepa_enabled);
    }

    #[test]
    fn pandora_score_community() {
        let s = PandoraScore::community();
        assert!(!s.official_status);
    }

    #[test]
    fn workflow_manifest_serde() {
        let m = WorkflowManifest {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            description: "test workflow".to_string(),
            steps: vec![WorkflowStep {
                name: "step1".to_string(),
                capability: "execution".to_string(),
                execution_profile: ExecutionProfile::Stateless,
                timeout_ms: 5000,
                retry_on_failure: true,
            }],
            retry_policy: RetryPolicy::default(),
            budget: WorkflowBudget::default(),
        };
        let json = serde_json::to_string(&m).unwrap();
        let _: WorkflowManifest = serde_json::from_str(&json).unwrap();
    }
}

// ============================================================
// Capability Constitutional Object
// ============================================================

/// Every capability becomes a constitutional object.
/// Capabilities define functionality that Genes implement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityManifest {
    pub name: String,
    pub kind: CapabilityKind,
    pub version: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub providers: Vec<String>,
    pub execution_profile: ExecutionProfile,
    pub evolution: EvolutionConfig,
}

/// The kind of capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CapabilityKind {
    Filesystem,
    Network,
    Browser,
    Shell,
    Provider,
    Memory,
    Gpu,
    Docker,
    Firecracker,
    Vm,
    Mcp,
    Plugin,
    Model,
    Embedding,
    Retrieval,
    Voice,
    Vision,
    Database,
    Compiler,
    Scheduler,
    Workflow,
    Checkpoint,
    Replay,
    Repair,
    #[default]
    Custom,
}

// ============================================================
// Gene Constitutional Metadata
// ============================================================

/// Every Gene must expose this metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub execution_mode: GeneExecutionMode,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub execution_profile: ExecutionProfile,
    pub evolution: EvolutionConfig,
    pub governance: GovernanceMetadata,
}

/// Gene execution modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum GeneExecutionMode {
    #[default]
    Chain,
    Hybrid,
    Independent,
}

// ============================================================
// Governance Metadata
// ============================================================

/// Governance metadata for constitutional objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernanceMetadata {
    pub requires_approval: bool,
    pub requires_audit: bool,
    pub trust_level: String,
    pub policy_refs: Vec<String>,
}

impl Default for GovernanceMetadata {
    fn default() -> Self {
        GovernanceMetadata {
            requires_approval: false,
            requires_audit: true,
            trust_level: "community".to_string(),
            policy_refs: vec![],
        }
    }
}

// ============================================================
// Engine Metadata
// ============================================================

/// Every engine must expose this metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineMetadata {
    pub owning_source_harness: String,
    pub owning_meta_harness: String,
    pub required_capabilities: Vec<String>,
    pub execution_profile: ExecutionProfile,
    pub gepa_support: bool,
    pub dsr_support: bool,
}

// ============================================================
// Universal Telemetry
// ============================================================

/// Universal telemetry model. Every constitutional
/// object uses this instead of custom telemetry types.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Telemetry {
    pub metrics: std::collections::BTreeMap<String, u64>,
    pub events: Vec<TelemetryEvent>,
    pub timestamps: TelemetryTimestamps,
    pub diagnostics: Vec<String>,
    pub errors: Vec<String>,
}

/// A single telemetry event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub name: String,
    pub timestamp_ms: u64,
    pub level: TelemetryLevel,
    pub message: String,
}

/// Telemetry event level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TelemetryLevel {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

/// Timestamps for telemetry.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TelemetryTimestamps {
    pub created_ms: u64,
    pub last_activity_ms: u64,
    pub last_health_check_ms: u64,
}

// ============================================================
// KUBER Palace Metadata
// ============================================================

/// Marketplace metadata for KUBER Palace publishing.
/// Every constitutional object exposes this.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuberMetadata {
    pub publisher: String,
    pub organization: String,
    pub visibility: Visibility,
    pub license: String,
    pub repository: String,
    pub documentation: String,
    pub examples: Vec<String>,
    pub official: bool,
    pub verified: bool,
    pub compatibility: Vec<String>,
    pub min_version: String,
    pub max_version: String,
}

/// Visibility for published objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Visibility {
    #[default]
    Public,
    Private,
    Organization,
}

impl Default for KuberMetadata {
    fn default() -> Self {
        KuberMetadata {
            publisher: String::new(),
            organization: String::new(),
            visibility: Visibility::Public,
            license: "MIT".to_string(),
            repository: String::new(),
            documentation: String::new(),
            examples: vec![],
            official: false,
            verified: false,
            compatibility: vec![],
            min_version: "0.1.0".to_string(),
            max_version: "2.0.0".to_string(),
        }
    }
}

// ============================================================
// Shadow Council
// ============================================================

/// Shadow Council is the constitutional authority.
/// Every decision flows through Shadow Council.
/// Shadow Council is permanent and never removable.
///
/// This is a contract-only type. No business logic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowCouncilDecision {
    pub decision_id: String,
    pub subject: String,
    pub action: CouncilAction,
    pub rationale: String,
    pub timestamp_ms: u64,
    pub trust_score: f64,
}

/// Actions the Shadow Council can take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CouncilAction {
    Approve,
    Reject,
    Quarantine,
    Escalate,
    Audit,
    #[default]
    Defer,
}

#[cfg(test)]
mod capability_tests {
    use super::*;

    #[test]
    fn capability_manifest_serde() {
        let m = CapabilityManifest {
            name: "filesystem".to_string(),
            kind: CapabilityKind::Filesystem,
            version: "1.0.0".to_string(),
            description: "File system access".to_string(),
            requirements: vec!["sandbox".to_string()],
            providers: vec!["phoenix".to_string()],
            execution_profile: ExecutionProfile::Stateless,
            evolution: EvolutionConfig::enabled(),
        };
        let json = serde_json::to_string(&m).unwrap();
        let _: CapabilityManifest = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn gene_manifest_serde() {
        let m = GeneManifest {
            name: "file_reader".to_string(),
            version: "1.0.0".to_string(),
            description: "Read files".to_string(),
            execution_mode: GeneExecutionMode::Chain,
            capabilities: vec!["filesystem".to_string()],
            dependencies: vec![],
            execution_profile: ExecutionProfile::Stateless,
            evolution: EvolutionConfig::enabled(),
            governance: GovernanceMetadata::default(),
        };
        let json = serde_json::to_string(&m).unwrap();
        let _: GeneManifest = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn gene_execution_modes() {
        assert_eq!(GeneExecutionMode::default(), GeneExecutionMode::Chain);
    }

    #[test]
    fn engine_metadata_serde() {
        let m = EngineMetadata {
            owning_source_harness: "phoenix".to_string(),
            owning_meta_harness: "execution".to_string(),
            required_capabilities: vec!["sandbox".to_string()],
            execution_profile: ExecutionProfile::Stateless,
            gepa_support: true,
            dsr_support: true,
        };
        let json = serde_json::to_string(&m).unwrap();
        let _: EngineMetadata = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn telemetry_default() {
        let t = Telemetry::default();
        assert!(t.metrics.is_empty());
        assert!(t.events.is_empty());
    }

    #[test]
    fn kuber_metadata_default() {
        let k = KuberMetadata::default();
        assert!(!k.official);
        assert_eq!(k.visibility, Visibility::Public);
    }

    #[test]
    fn shadow_council_decision_serde() {
        let d = ShadowCouncilDecision {
            decision_id: "d1".to_string(),
            subject: "phoenix".to_string(),
            action: CouncilAction::Approve,
            rationale: "constitutional".to_string(),
            timestamp_ms: 0,
            trust_score: 1.0,
        };
        let json = serde_json::to_string(&d).unwrap();
        let _: ShadowCouncilDecision = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn capability_kinds() {
        assert_eq!(CapabilityKind::default(), CapabilityKind::Custom);
    }
}
