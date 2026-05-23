use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionScore {
    pub score_id: String,

    pub target_graph: String,

    pub target_mutation: String,

    pub execution_score: f32,

    pub governance_score: f32,

    pub replay_confidence: f32,

    pub mutation_stability: f32,

    pub evaluator: String,

    pub timestamp: String,
}
