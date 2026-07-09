//! Execution Plan — immutable specification for a single execution.
//!
//! Every execution starts from a plan. The plan encodes what to do, how
//! to evaluate it, when to stop, and which providers/policies to use.
//! This makes execution deterministic, replayable, and inspectable.

use serde::{Serialize, Deserialize};

/// What triggers this execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ExecutionTrigger {
    #[default]
    Manual,    // User invoked via CLI
    Scheduled, // Timer/cron job
    Event,     // External event webhook
}

/// How the controller evaluates completion.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ControlStrategy {
    #[default]
    SingleShot, // Run once, return
    Closed,     // Evaluate + retry until stop condition
    Open,       // LLM decides whether to continue
    Human,      // Wait for human approval between steps
    Autonomous, // Full autonomy with governance oversight
}

/// How work is distributed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum ExecutionMode {
    #[default]
    Single,   // One runner
    Parallel, // Multiple runners, same plan
    Fleet,    // Distributed runners across nodes
}

/// When to stop (for Closed/Open strategies).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum StopCondition {
    #[default]
    GoalMet,      // Evaluator says goal achieved
    MaxAttempts,  // Hit retry limit
    ManualStop,   // User interrupted
    Timeout,      // Deadline exceeded
    Governance,   // Policy rejected further execution
}

/// Who provides evaluation for goal-based execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum EvaluatorKind {
    #[default]
    None,          // No evaluation (single-shot)
    RustTests,     // cargo test
    PythonTests,   // pytest
    OutputMatch,   // Output matches expected string/regex
    Custom(String),// Domain-specific evaluator
}

/// An immutable execution plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub instruction: String,
    pub workflow: String,
    pub execution_mode: ExecutionMode,
    pub control_strategy: ControlStrategy,
    pub trigger: ExecutionTrigger,
    pub evaluator: EvaluatorKind,
    pub max_attempts: u32,
    pub stop_condition: StopCondition,
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
            max_attempts: 1,
            stop_condition: StopCondition::GoalMet,
            provider_policy: "default".into(),
            approval_required: false,
            sandbox_level: 0,
        }
    }
}

impl ExecutionPlan {
    /// Create a new plan for a single-shot execution.
    pub fn single_shot(instruction: &str) -> Self {
        Self { instruction: instruction.into(), ..Default::default() }
    }

    /// Create a goal-based plan with retry.
    pub fn goal_based(instruction: &str, evaluator: EvaluatorKind, max_attempts: u32) -> Self {
        Self {
            instruction: instruction.into(),
            control_strategy: ControlStrategy::Closed,
            evaluator,
            max_attempts,
            ..Default::default()
        }
    }

    /// Human-approval plan — pauses for approval between stages.
    pub fn with_approval(mut self, required: bool) -> Self {
        if required { self.control_strategy = ControlStrategy::Human; }
        self.approval_required = required;
        self
    }

    /// Set sandbox level.
    pub fn sandbox(mut self, level: u8) -> Self {
        self.sandbox_level = level;
        self
    }
}
