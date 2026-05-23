use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PandoraEvent {
    pub event_id: String,

    pub event_type: String,

    pub timestamp: String,

    pub source_gene: String,

    pub payload: serde_json::Value,
}
