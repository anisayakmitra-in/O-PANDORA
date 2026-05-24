use serde::{Deserialize, Serialize};

use crate::swarm_phenotype::SwarmPhenotype;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstinctResponse {
    pub instinct: String,

    pub action: String,
}

pub struct InstinctEngine;

impl InstinctEngine {
    pub fn evaluate(phenotype: &SwarmPhenotype) -> Vec<InstinctResponse> {
        let mut responses = Vec::new();

        if phenotype.survivability_score < 0.85 {
            responses.push(InstinctResponse {
                instinct: "self_preservation".into(),

                action: "trigger_repair".into(),
            });
        }

        if phenotype.execution_bias == "aggressive-scaling" {
            responses.push(InstinctResponse {
                instinct: "expansion".into(),

                action: "spawn_execution_swarm".into(),
            });
        }

        println!("[INSTINCT] responses={}", responses.len());

        responses
    }
}
