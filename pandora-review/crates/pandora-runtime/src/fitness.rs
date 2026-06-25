use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessEvaluation {
    pub evaluation_id: String,

    pub candidate_id: String,

    pub replay_score: f32,

    pub entropy_score: f32,

    pub retrieval_score: f32,

    pub survivability_score: f32,

    pub governance_score: f32,

    pub final_score: f32,
}

pub struct FitnessEngine;

impl FitnessEngine {
    pub fn evaluate(
        candidate_id: impl Into<String>,

        replay_score: f32,

        entropy_score: f32,

        retrieval_score: f32,

        survivability_score: f32,

        governance_score: f32,
    ) -> FitnessEvaluation {
        let final_score = replay_score * 0.20
            + entropy_score * 0.15
            + retrieval_score * 0.20
            + survivability_score * 0.30
            + governance_score * 0.15;

        FitnessEvaluation {
            evaluation_id: "fitness_001".into(),

            candidate_id: candidate_id.into(),

            replay_score,

            entropy_score,

            retrieval_score,

            survivability_score,

            governance_score,

            final_score,
        }
    }
}
