use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivabilityCandidate {
    pub runtime: String,

    pub stability: f64,

    pub recovery_rate: f64,

    pub resource_efficiency: f64,

    pub mutation_resistance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivabilityAssessment {
    pub runtime: String,

    pub survivability_score: f64,

    pub resilient: bool,
}

pub struct ExecutionSurvivabilityEngine;

impl ExecutionSurvivabilityEngine {
    pub fn evaluate(candidate: &SurvivabilityCandidate) -> SurvivabilityAssessment {
        println!("[SURVIVABILITY] evaluating {}", candidate.runtime);

        let score = (candidate.stability * 0.35)
            + (candidate.recovery_rate * 0.30)
            + (candidate.resource_efficiency * 0.15)
            + (candidate.mutation_resistance * 0.20);

        let resilient = score > 0.80;

        SurvivabilityAssessment {
            runtime: candidate.runtime.clone(),

            survivability_score: score,

            resilient,
        }
    }
}
