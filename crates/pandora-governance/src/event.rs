use std::time::SystemTime;
use std::sync::Arc;

use serde::Serialize;
use uuid::Uuid;

use crate::context::ExecutionContext;

#[derive(Debug, Clone, Serialize)]
pub enum ExecutionEventKind {

    Started,

    Stdout {
        line: String,
    },

    Stderr {
        line: String,
    },

    Finished {
        exit_code: i64,
    },

    TimedOut,

    Cancelled,

    GovernanceDenied {
        reason: String,
    },

    SandboxProvisioned {
        container_id: String,
    },

    SandboxDestroyed {
        container_id: String,
    },

    HostExecutionStarted,

    SubagentSpawned {
        child_trace_id: Uuid,
    },

    KillSwitchTriggered,
}

#[derive(Debug, Clone)]
pub struct ExecutionEvent {

    pub timestamp:
        SystemTime,

    pub context:
        Arc<ExecutionContext>,

    pub kind:
        ExecutionEventKind,
}

impl ExecutionEvent {

    pub fn new(
        context: Arc<ExecutionContext>,
        kind: ExecutionEventKind,
    ) -> Self {

        Self {

            timestamp:
                SystemTime::now(),

            context,

            kind,
        }
    }
}
