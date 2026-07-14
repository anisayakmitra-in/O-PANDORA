//! Constitutional Execution Runtime — Runtime Context and Execution Properties.
//!
//! `RuntimeContext` is the foundation for all execution. Every subsystem
//! receives `&RuntimeContext` instead of random parameters. It exists only
//! while something runs.
//!
//! `ExecutionProperties` standardize execution behavior across all engines.
//! No subsystem invents its own settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Execution Properties ──

/// Memory persistence strategy for this execution.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum MemoryMode {
    #[default]
    Local,
    ANUBIS,
    Hybrid,
}

/// How the execution is scheduled.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub enum ExecutionMode {
    #[default]
    Single,
    Parallel,
}

/// When the execution should stop.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum ControlStrategy {
    #[default]
    SingleShot,
    Closed,
    Open,
    Human,
    Autonomous,
}

/// Safety level for execution.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum SafetyLevel {
    Low,
    #[default]
    Medium,
    High,
    Maximum,
}

/// Execution backend type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum ExecutionBackend {
    #[default]
    Native,
    Docker,
    WASM,
    Firecracker,
    Remote,
}

/// Approval mode for execution decisions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum ApprovalMode {
    #[default]
    Auto,
    Confirm,
    Review,
    Vote,
}

/// Provider selection policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ProviderPolicy {
    #[default]
    Auto,
    PreferLocal,
    PreferCloud,
    Specific(String),
}

/// Retry strategy for failures.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RetryStrategy {
    None,
    Fixed(u32),
    Exponential { max_retries: u32, base_delay_ms: u64 },
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self::Fixed(3)
    }
}

/// Context window strategy.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ContextWindowStrategy {
    Fixed(usize),
    Dynamic { max_tokens: usize, compression_ratio: f64 },
}

impl Default for ContextWindowStrategy {
    fn default() -> Self {
        Self::Fixed(128_000)
    }
}

/// Complete execution properties — standardized across all subsystems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProperties {
    pub memory_mode: MemoryMode,
    pub exec_mode: ExecutionMode,
    pub control: ControlStrategy,
    pub safety_level: SafetyLevel,
    pub execution_backend: ExecutionBackend,
    pub approval_mode: ApprovalMode,
    pub provider_policy: ProviderPolicy,
    pub retry_strategy: RetryStrategy,
    pub context_strategy: ContextWindowStrategy,
    pub reasoning_depth: u32,
    pub parallelism: u32,
    pub checkpoint_interval_secs: u64,
    pub record_execution: bool,
    pub telemetry_level: u8,
    pub cost_budget: f64,
    pub latency_target_ms: u64,
    pub deadline_secs: u64,
}

impl Default for ExecutionProperties {
    fn default() -> Self {
        Self {
            memory_mode: MemoryMode::default(),
            exec_mode: ExecutionMode::default(),
            control: ControlStrategy::default(),
            safety_level: SafetyLevel::default(),
            execution_backend: ExecutionBackend::default(),
            approval_mode: ApprovalMode::default(),
            provider_policy: ProviderPolicy::default(),
            retry_strategy: RetryStrategy::default(),
            context_strategy: ContextWindowStrategy::default(),
            reasoning_depth: 3,
            parallelism: 1,
            checkpoint_interval_secs: 300,
            record_execution: true,
            telemetry_level: 2,
            cost_budget: 1.0,
            latency_target_ms: 30_000,
            deadline_secs: 3600,
        }
    }
}

// ── Runtime Context ──

/// A unique execution identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExecutionId(pub String);

impl ExecutionId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        ExecutionId(format!("exec-{:016x}", COUNTER.fetch_add(1, Ordering::Relaxed)))
    }
}

impl Default for ExecutionId {
    fn default() -> Self {
        Self::new()
    }
}

/// The Runtime Context — foundation for all execution.
///
/// Every subsystem receives `&RuntimeContext`. It exists only while
/// something runs. It is NOT memory. Memory is persistence. Context
/// is execution state.
#[derive(Debug, Clone)]
pub struct RuntimeContext {
    pub execution_id: ExecutionId,
    pub session_id: String,
    pub project_id: String,
    pub properties: ExecutionProperties,
    pub variables: HashMap<String, String>,
    pub active_capabilities: Vec<String>,
    pub leased_services: Vec<String>,
    pub provider_selection: Option<String>,
    pub model_context: String,
    pub checkpoints: Vec<Checkpoint>,
    pub artifacts: Vec<String>,
    pub telemetry: Vec<String>,
    pub budget_remaining: f64,
    pub deadline: std::time::Instant,
    pub start_time: std::time::Instant,
}

impl RuntimeContext {
    pub fn new(session_id: impl Into<String>, project_id: impl Into<String>) -> Self {
        Self {
            execution_id: ExecutionId::new(),
            session_id: session_id.into(),
            project_id: project_id.into(),
            properties: ExecutionProperties::default(),
            variables: HashMap::new(),
            active_capabilities: Vec::new(),
            leased_services: Vec::new(),
            provider_selection: None,
            model_context: String::new(),
            checkpoints: Vec::new(),
            artifacts: Vec::new(),
            telemetry: Vec::new(),
            budget_remaining: 1.0,
            deadline: std::time::Instant::now() + std::time::Duration::from_secs(3600),
            start_time: std::time::Instant::now(),
        }
    }

    pub fn with_properties(mut self, props: ExecutionProperties) -> Self {
        self.properties = props;
        self
    }

    pub fn elapsed_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    pub fn is_expired(&self) -> bool {
        std::time::Instant::now() >= self.deadline
    }

    pub fn set_variable(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }

    pub fn get_variable(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(String::as_str)
    }

    pub fn add_artifact(&mut self, artifact: impl Into<String>) {
        self.artifacts.push(artifact.into());
    }

    pub fn budget_used(&self) -> f64 {
        self.budget_remaining
    }

    pub fn record_telemetry(&mut self, line: impl Into<String>) {
        self.telemetry.push(line.into());
    }
}

/// A checkpoint for execution state snapshots.
#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub id: String,
    pub label: String,
    pub timestamp: std::time::Instant,
    pub variables: HashMap<String, String>,
    pub artifacts: Vec<String>,
}

impl Checkpoint {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: format!("cp-{:016x}", rand::random::<u64>()),
            label: label.into(),
            timestamp: std::time::Instant::now(),
            variables: HashMap::new(),
            artifacts: Vec::new(),
        }
    }
}

// ── Session (legacy, linked to RuntimeContext) ──

/// A session groups multiple executions under one conversation.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Session {
    pub session_id: String,
    pub project_id: String,
    pub properties: ExecutionProperties,
    pub context: RuntimeContext,
}

#[allow(dead_code)]
impl Session {
    pub fn new(session_id: impl Into<String>, project_id: impl Into<String>) -> Self {
        let s: String = session_id.into();
        let p: String = project_id.into();
        let context = RuntimeContext::new(s.clone(), p.clone());
        Self {
            session_id: s,
            project_id: p,
            properties: ExecutionProperties::default(),
            context,
        }
    }

    pub fn create_execution(&self, properties: Option<ExecutionProperties>) -> RuntimeContext {
        let mut ctx = RuntimeContext::new(&self.session_id, &self.project_id);
        ctx.properties = properties.unwrap_or_else(|| self.properties.clone());
        ctx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_id_unique() {
        let a = ExecutionId::new();
        let b = ExecutionId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn runtime_context_new() {
        let ctx = RuntimeContext::new("session-1", "project-pandora");
        assert_eq!(ctx.session_id, "session-1");
        assert_eq!(ctx.project_id, "project-pandora");
        assert!(!ctx.is_expired());
    }

    #[test]
    fn execution_properties_defaults() {
        let props = ExecutionProperties::default();
        assert_eq!(props.memory_mode, MemoryMode::Local);
        assert_eq!(props.exec_mode, ExecutionMode::Single);
        assert_eq!(props.safety_level, SafetyLevel::Medium);
    }

    #[test]
    fn context_variables() {
        let mut ctx = RuntimeContext::new("s", "p");
        ctx.set_variable("PROJECT", "pandora");
        ctx.set_variable("MODEL", "qwen");
        assert_eq!(ctx.get_variable("PROJECT"), Some("pandora"));
        assert_eq!(ctx.get_variable("MODEL"), Some("qwen"));
        assert_eq!(ctx.get_variable("NONEXIST"), None);
    }

    #[test]
    fn session_creates_execution() {
        let session = Session::new("chat-1", "project-x");
        let ctx = session.create_execution(None);
        assert_eq!(ctx.session_id, "chat-1");
        assert_eq!(ctx.project_id, "project-x");
    }

    #[test]
    fn custom_properties_override() {
        let props = ExecutionProperties {
            exec_mode: ExecutionMode::Single,
            control: ControlStrategy::Open,
            reasoning_depth: 5,
            ..Default::default()
        };
        let session = Session::new("s", "p");
        let ctx = session.create_execution(Some(props));
        assert_eq!(ctx.properties.exec_mode, ExecutionMode::Single);
        assert_eq!(ctx.properties.reasoning_depth, 5);
    }

    #[test]
    fn checkpoint_creation() {
        let cp = Checkpoint::new("before-execution");
        assert!(cp.id.starts_with("cp-"));
        assert_eq!(cp.label, "before-execution");
    }

    #[test]
    fn elapsed_time() {
        let ctx = RuntimeContext::new("s", "p");
        assert!(ctx.elapsed_secs() < 5);
    }

    #[test]
    fn memory_mode_display() {
        assert_eq!(format!("{:?}", MemoryMode::Local), "Local");
        assert_eq!(format!("{:?}", MemoryMode::ANUBIS), "ANUBIS");
    }
}
