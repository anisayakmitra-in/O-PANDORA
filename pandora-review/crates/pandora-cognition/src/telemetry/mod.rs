use std::time::{
    Duration,
    SystemTime,
};

use serde::{
    Serialize,
    Deserialize,
};

use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CognitionTelemetry {

    pub trace_id:
        Uuid,

    pub module_name:
        String,

    pub pipeline_name:
        Option<String>,

    pub started_at:
        SystemTime,

    pub duration:
        Duration,

    pub success:
        bool,

    pub score:
        Option<f32>,

    pub tokens_used:
        Option<u64>,

    pub provider:
        Option<String>,

    pub model:
        Option<String>,

    pub notes:
        Vec<String>,
}

pub mod recorder;
