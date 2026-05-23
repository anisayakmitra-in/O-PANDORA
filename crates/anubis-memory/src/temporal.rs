use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalMemory {
    pub memory_id: String,

    pub timestamp: String,

    pub sequence: u64,

    pub recency_score: f32,
}
