use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalienceScore {
    pub memory_id: String,

    pub replay_frequency: f32,

    pub governance_importance: f32,

    pub graph_centrality: f32,

    pub final_score: f32,
}
