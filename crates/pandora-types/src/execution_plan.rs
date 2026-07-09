//! ExecutionPlan — immutable execution specification.
//! ExecutionState — mutable runtime state.
//! ExecutionOutcome — immutable record of what happened.

use serde::{Serialize, Deserialize};
use std::time::Duration;

// ── Execution Plan (immutable) ──

/// What triggers this execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ExecutionTrigger { #[default] Manual, Scheduled, Event }

/// How the controller evaluates completion.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ControlStrategy { #[default] SingleShot, Closed, Open, Human, Autonomous }

/// How work is distributed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ExecutionMode { #[default] Single, Parallel, Fleet }

/// When to stop. Multiple conditions can be combined.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StopCondition { GoalMet, MaxAttempts(u32), ManualStop, Timeout(u64), Governance }

/// Who evaluates goal-based execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum EvaluatorKind { #[default] None, RustTests, PythonTests, OutputMatch, Custom(String) }

/// An immutable execution plan — never changes during execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub sandbox_level: u8,
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
            sandbox_level: 0,
        }
    }
}

impl ExecutionPlan {
    pub fn single_shot(instruction: &str) -> Self { Self { instruction: instruction.into(), ..Default::default() } }
    pub fn goal_based(instruction: &str, evaluator: EvaluatorKind, max_attempts: u32) -> Self {
        Self {
            instruction: instruction.into(),
            control_strategy: ControlStrategy::Closed,
            evaluator,
            stop_conditions: vec![StopCondition::GoalMet, StopCondition::MaxAttempts(max_attempts)],
            ..Default::default()
        }
    }
    pub fn with_approval(mut self, required: bool) -> Self { self.approval_required = required; self.control_strategy = ControlStrategy::Human; self }
    pub fn sandbox(mut self, level: u8) -> Self { self.sandbox_level = level; self }
}

// ── Execution State (mutable) ──

/// Current runtime state — changes during execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ExecutionStatus { #[default] Pending, Running, Paused, Completed, Failed, Cancelled, Rejected }

// ── Execution Outcome (immutable record) ──

/// What happened — stable record produced after execution completes.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
