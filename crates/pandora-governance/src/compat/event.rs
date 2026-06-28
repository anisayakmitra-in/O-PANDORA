use std::sync::Arc;

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::context::ExecutionContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionEventKind {
    ExecutionStarted { command: Vec<String> },

    ExecutionCompleted { exit_code: i64 },

    ExecutionFailed { reason: String },

    HostExecutionStarted { command: Vec<String> },

    SandboxExecutionStarted { command: Vec<String> },

    Stdout { line: String },

    Stderr { line: String },

    GovernanceViolation { reason: String },

    WatchdogTriggered { reason: String },

    Cancelled,
}

#[derive(Debug, Clone)]
pub struct ExecutionEvent {
    pub timestamp: SystemTime,

    pub trace_id: uuid::Uuid,

    pub parent_trace_id: Option<uuid::Uuid>,

    pub tier: String,

    pub kind: ExecutionEventKind,
}

impl ExecutionEvent {
    pub fn new(context: Arc<ExecutionContext>, kind: ExecutionEventKind) -> Self {
        Self {
            timestamp: SystemTime::now(),

            trace_id: context.trace_id,

            parent_trace_id: context.parent_trace_id,

            tier: format!("{:?}", context.tier),

            kind,
        }
    }
}
