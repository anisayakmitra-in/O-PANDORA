//! Universal constitutional models.
//!
//! Canonical implementations of Health, Lifecycle, ExecutionProfile,
//! PandoraScore, DebugPipeline, and related metadata types. Every
//! constitutional object must use these models — do not redefine them.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The canonical health model for every constitutional object.
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
        matches!(self, Self::Ready | Self::Healthy | Self::Busy)
    }
    pub fn is_operational(self) -> bool {
        matches!(
            self,
            Self::Ready | Self::Healthy | Self::Busy | Self::Degraded | Self::Recovering
        )
    }
}

impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ready => "ready",
                Self::Healthy => "healthy",
                Self::Busy => "busy",
                Self::Degraded => "degraded",
                Self::Recovering => "recovering",
                Self::Repairing => "repairing",
                Self::Restarting => "restarting",
                Self::Quarantined => "quarantined",
                Self::Offline => "offline",
                Self::Archived => "archived",
            }
        )
    }
}

/// The canonical lifecycle model for every constitutional object.
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
        matches!(self, Self::Ready | Self::Running | Self::Paused)
    }
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Archived | Self::Deprecated | Self::Deleted)
    }
}

impl std::fmt::Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Created => "created",
                Self::Installed => "installed",
                Self::Loaded => "loaded",
                Self::Initialized => "initialized",
                Self::Ready => "ready",
                Self::Running => "running",
                Self::Paused => "paused",
                Self::Recovering => "recovering",
                Self::Updating => "updating",
                Self::Stopping => "stopping",
                Self::Stopped => "stopped",
                Self::Archived => "archived",
                Self::Deprecated => "deprecated",
                Self::Deleted => "deleted",
            }
        )
    }
}

/// Execution classification for constitutional objects.
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

/// Debug pipeline phase for source harnesses.
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

/// GEPA/DSR configuration for constitutional objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvolutionConfig {
    pub gepa_enabled: bool,
    pub dsr_enabled: bool,
}

impl EvolutionConfig {
    pub fn enabled() -> Self {
        Self {
            gepa_enabled: true,
            dsr_enabled: true,
        }
    }
    pub fn disabled() -> Self {
        Self {
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

/// Quality metadata for published objects.
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
        Self {
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
        Self {
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

/// A Workflow is a constitutional object between Engine and Capability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub retry_policy: RetryPolicy,
    pub budget: WorkflowBudget,
}

/// A single workflow step.
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
        Self {
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
        Self {
            max_duration_ms: 60_000,
            max_cost_cents: 100,
            max_memory_mb: 1024,
        }
    }
}

/// Every capability becomes a constitutional object.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum GeneExecutionMode {
    #[default]
    Chain,
    Hybrid,
    Independent,
}

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
        Self {
            requires_approval: false,
            requires_audit: true,
            trust_level: "community".into(),
            policy_refs: vec![],
        }
    }
}

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

/// Universal telemetry model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Telemetry {
    pub metrics: BTreeMap<String, u64>,
    pub events: Vec<TelemetryEvent>,
    pub timestamps: TelemetryTimestamps,
    pub diagnostics: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub name: String,
    pub timestamp_ms: u64,
    pub level: TelemetryLevel,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TelemetryLevel {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TelemetryTimestamps {
    pub created_ms: u64,
    pub last_activity_ms: u64,
    pub last_health_check_ms: u64,
}

/// Marketplace metadata for KUBER K-O Palace publishing.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Visibility {
    #[default]
    Public,
    Private,
    Organization,
}

impl Default for KuberMetadata {
    fn default() -> Self {
        Self {
            publisher: String::new(),
            organization: String::new(),
            visibility: Visibility::Public,
            license: "MIT".into(),
            repository: String::new(),
            documentation: String::new(),
            examples: vec![],
            official: false,
            verified: false,
            compatibility: vec![],
            min_version: "0.1.0".into(),
            max_version: "2.0.0".into(),
        }
    }
}

/// Shadow Council decision contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowCouncilDecision {
    pub decision_id: String,
    pub subject: String,
    pub action: CouncilAction,
    pub rationale: String,
    pub timestamp_ms: u64,
    pub trust_score: f64,
}

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
mod tests {
    use super::*;

    #[test]
    fn health_dispatchable() {
        assert!(Health::Ready.is_dispatchable());
        assert!(!Health::Degraded.is_dispatchable());
    }
    #[test]
    fn health_operational() {
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
        assert!(!Lifecycle::Created.is_active());
    }
    #[test]
    fn lifecycle_terminal() {
        assert!(Lifecycle::Archived.is_terminal());
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
    fn evolution_config() {
        assert!(EvolutionConfig::enabled().gepa_enabled);
        assert!(!EvolutionConfig::disabled().gepa_enabled);
    }
    #[test]
    fn pandora_score() {
        assert!(PandoraScore::official().official_status);
        assert!(!PandoraScore::community().official_status);
    }
    #[test]
    fn workflow_serde() {
        let m = WorkflowManifest {
            name: "test".into(),
            version: "1.0.0".into(),
            description: "test".into(),
            steps: vec![],
            retry_policy: RetryPolicy::default(),
            budget: WorkflowBudget::default(),
        };
        let j = serde_json::to_string(&m).expect("serialization");
        let _: WorkflowManifest = serde_json::from_str(&j).expect("serialization");
    }
    #[test]
    fn capability_serde() {
        let m = CapabilityManifest {
            name: "filesystem".into(),
            kind: CapabilityKind::Filesystem,
            version: "1.0.0".into(),
            description: "fs".into(),
            requirements: vec![],
            providers: vec![],
            execution_profile: ExecutionProfile::Stateless,
            evolution: EvolutionConfig::enabled(),
        };
        let j = serde_json::to_string(&m).expect("serialization");
        let _: CapabilityManifest = serde_json::from_str(&j).expect("serialization");
    }
    #[test]
    fn gene_execution_modes() {
        assert_eq!(GeneExecutionMode::default(), GeneExecutionMode::Chain);
    }
    #[test]
    fn kuber_default() {
        assert_eq!(KuberMetadata::default().visibility, Visibility::Public);
    }
    #[test]
    fn shadow_council_serde() {
        let d = ShadowCouncilDecision {
            decision_id: "d1".into(),
            subject: "phoenix".into(),
            action: CouncilAction::Approve,
            rationale: "constitutional".into(),
            timestamp_ms: 0,
            trust_score: 1.0,
        };
        let j = serde_json::to_string(&d).expect("serialization");
        let _: ShadowCouncilDecision = serde_json::from_str(&j).expect("serialization");
    }
}
