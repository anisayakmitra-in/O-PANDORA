//! Execution contracts — the three immutable/mutable/stable records
//! that form Pandora's execution ABI.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ── Sandbox Level ──

/// Sandbox isolation level for execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub enum SandboxLevel {
    #[default]
    None,
    /// Network restricted, read-only filesystem.
    Restricted,
    /// Fully isolated sandbox (Firecracker, nsjail, etc.).
    Isolated,
}

// ── Execution Budget ──

/// Concrete resource constraints the ExecutionController enforces.
///
/// Carried inside `ExecutionPlan`. Gives the controller deterministic
/// bounds for provider selection, retries, parallelism, and approval
/// policies — no ad-hoc logic needed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionBudget {
    /// Maximum wall-clock duration for the entire execution.
    pub max_duration: Duration,
    /// Maximum cost in USD (provider-dependent, 0 = unlimited).
    pub max_cost_usd: f32,
    /// Maximum total tokens across all provider calls.
    pub max_tokens: usize,
    /// Maximum retry attempts per stage (0 = no retries).
    pub max_retries: u32,
    /// Maximum parallel branches (0 = sequential only).
    pub max_parallelism: usize,
    /// Sandbox isolation level.
    pub sandbox_level: SandboxLevel,
}

impl Default for ExecutionBudget {
    fn default() -> Self {
        Self {
            max_duration: Duration::from_secs(300), // 5 minutes
            max_cost_usd: 0.0,                      // unlimited
            max_tokens: 100_000,
            max_retries: 3,
            max_parallelism: 1,
            sandbox_level: SandboxLevel::None,
        }
    }
}

impl ExecutionBudget {
    pub fn unlimited() -> Self {
        Self {
            max_duration: Duration::MAX,
            max_cost_usd: 0.0,
            max_tokens: usize::MAX,
            max_retries: 0,
            max_parallelism: 0,
            sandbox_level: SandboxLevel::None,
        }
    }

    pub fn strict() -> Self {
        Self {
            max_duration: Duration::from_secs(60),
            max_cost_usd: 0.01,
            max_tokens: 10_000,
            max_retries: 1,
            max_parallelism: 0,
            sandbox_level: SandboxLevel::Restricted,
        }
    }

    pub fn sandbox(mut self, level: SandboxLevel) -> Self {
        self.sandbox_level = level;
        self
    }
    pub fn retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }
    pub fn budget(mut self, usd: f32) -> Self {
        self.max_cost_usd = usd;
        self
    }
}

// ── Enums ──

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ExecutionTrigger {
    #[default]
    Manual,
    Scheduled,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ControlStrategy {
    #[default]
    SingleShot,
    Closed,
    Open,
    Human,
    Autonomous,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ExecutionMode {
    #[default]
    Single,
    Parallel,
    Fleet,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StopCondition {
    GoalMet,
    MaxAttempts(u32),
    ManualStop,
    Timeout(u64),
    Governance,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum EvaluatorKind {
    #[default]
    None,
    RustTests,
    PythonTests,
    OutputMatch,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ExecutionStatus {
    #[default]
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
    Rejected,
}

// ── Execution Plan (immutable) ──

/// An immutable execution plan — never changes during execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub instruction: String,
    pub workflow: String,
    pub execution_mode: ExecutionMode,
    pub control_strategy: ControlStrategy,
    pub trigger: ExecutionTrigger,
    pub evaluator: EvaluatorKind,
    pub stop_conditions: Vec<StopCondition>,
    pub provider_policy: String,
    pub approval_required: bool,
    /// Resource budget the controller must respect.
    pub budget: ExecutionBudget,
}

impl Default for ExecutionPlan {
    fn default() -> Self {
        Self {
            instruction: String::new(),
            workflow: "default".into(),
            execution_mode: ExecutionMode::Single,
            control_strategy: ControlStrategy::SingleShot,
            trigger: ExecutionTrigger::Manual,
            evaluator: EvaluatorKind::None,
            stop_conditions: vec![StopCondition::GoalMet],
            provider_policy: "default".into(),
            approval_required: false,
            budget: ExecutionBudget::default(),
        }
    }
}

impl ExecutionPlan {
    pub fn single_shot(instruction: &str) -> Self {
        Self {
            instruction: instruction.into(),
            ..Default::default()
        }
    }
    pub fn goal_based(instruction: &str, evaluator: EvaluatorKind, max_attempts: u32) -> Self {
        Self {
            instruction: instruction.into(),
            control_strategy: ControlStrategy::Closed,
            evaluator,
            stop_conditions: vec![
                StopCondition::GoalMet,
                StopCondition::MaxAttempts(max_attempts),
            ],
            ..Default::default()
        }
    }
    pub fn with_approval(mut self, required: bool) -> Self {
        self.approval_required = required;
        self.control_strategy = ControlStrategy::Human;
        self
    }
    pub fn with_budget(mut self, budget: ExecutionBudget) -> Self {
        self.budget = budget;
        self
    }
}

// ── Execution State (mutable) ──

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionState {
    pub session_id: String,
    pub current_stage: String,
    pub attempt: u32,
    pub retries: u32,
    pub current_provider: String,
    pub current_harness: String,
    pub status: ExecutionStatus,
    pub elapsed_ms: u64,
    pub started_at: String,
}
impl Default for ExecutionState {
    fn default() -> Self {
        Self {
            session_id: String::new(),
            current_stage: "init".into(),
            attempt: 0,
            retries: 0,
            current_provider: "none".into(),
            current_harness: "none".into(),
            status: ExecutionStatus::Pending,
            elapsed_ms: 0,
            started_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

// ── Execution Outcome (immutable record) ──

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionOutcome {
    pub session_id: String,
    pub status: ExecutionStatus,
    pub attempts: u32,
    pub retries: u32,
    pub evaluator_result: String,
    pub governance_result: String,
    pub provider_used: String,
    pub harness_used: String,
    pub genes_used: Vec<String>,
    pub artifacts: Vec<String>,
    pub duration_ms: u64,
    pub output: String,
}
impl ExecutionOutcome {
    pub fn from_state(state: &ExecutionState) -> Self {
        Self {
            session_id: state.session_id.clone(),
            status: state.status.clone(),
            attempts: state.attempt,
            retries: state.retries,
            evaluator_result: String::new(),
            governance_result: String::new(),
            provider_used: state.current_provider.clone(),
            harness_used: state.current_harness.clone(),
            genes_used: Vec::new(),
            artifacts: Vec::new(),
            duration_ms: state.elapsed_ms,
            output: String::new(),
        }
    }
}
