use serde::Serialize;

use std::time::SystemTime;

#[derive(Debug, Clone, Serialize)]
pub struct AuditEvent {

    pub timestamp:
        SystemTime,

    pub trace_id:
        String,

    pub tier:
        String,

    pub command:
        Vec<String>,

    pub environment:
        String,

    pub operator_id:
        Option<String>,

    pub outcome:
        String,
}

