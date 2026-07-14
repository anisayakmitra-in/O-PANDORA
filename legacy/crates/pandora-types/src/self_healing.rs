//! Self-healing runtime types — detects failures and orchestrates automatic recovery.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A detected failure in the execution pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectedFailure {
    pub failure_id: String,
    pub kind: FailureKind,
    pub source: String,
    pub message: String,
    pub timestamp_ms: u64,
    pub retryable: bool,
}

/// Kinds of failures the self-healing system detects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FailureKind {
    Timeout,
    Deadlock,
    ResourceExhaustion,
    DependencyFailure,
    ProviderFailure,
    SandboxFailure,
    WorkflowFailure,
    GeneFailure,
    BudgetExhaustion,
    PermissionDenied,
    InternalError,
}

/// An action taken to recover from a failure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryAction {
    pub action_id: String,
    pub kind: RecoveryKind,
    pub target: String,
    pub timestamp_ms: u64,
    pub result: RecoveryResult,
}

/// Kinds of recovery actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecoveryKind {
    Checkpoint,
    Rollback,
    Repair,
    Retry,
    Fallback,
    AlternateProvider,
    AlternateSandbox,
    BranchExecution,
    Escalate,
}

/// Result of a recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecoveryResult {
    Pending,
    Success,
    Failed,
    Skipped,
}

/// A self-healing session tracks failures and recovery actions for an execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealingSession {
    pub session_id: String,
    pub failures: Vec<DetectedFailure>,
    pub actions: Vec<RecoveryAction>,
    pub max_retries: u32,
    pub retry_count: u32,
    pub metrics: BTreeMap<String, u64>,
}

impl HealingSession {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            failures: vec![],
            actions: vec![],
            max_retries: 3,
            retry_count: 0,
            metrics: BTreeMap::new(),
        }
    }

    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
}
