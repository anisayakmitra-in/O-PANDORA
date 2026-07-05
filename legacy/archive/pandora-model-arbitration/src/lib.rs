//! Pandora Model Arbitration — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCandidate {
    pub provider: String,

    pub reasoning_score: f64,

    pub speed_score: f64,

    pub memory_score: f64,

    pub tool_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrationDecision {
    pub selected_provider: String,

    pub final_score: f64,

    pub rationale: String,
}

pub struct MultiModelArbitrationEngine;

impl MultiModelArbitrationEngine {
    pub fn select(candidates: &[ModelCandidate], workload: &str) -> Option<ArbitrationDecision> {
        println!("[ARBITRATION] workload={}", workload);

        let mut best_score = 0.0;

        let mut selected = None;

        for candidate in candidates {
            println!("[ARBITRATION] evaluating {}", candidate.provider);

            let mut score = (candidate.reasoning_score * 0.35)
                + (candidate.speed_score * 0.20)
                + (candidate.memory_score * 0.20)
                + (candidate.tool_score * 0.25);

            if workload.contains("reasoning") {
                score += candidate.reasoning_score * 0.15;
            }

            if workload.contains("repair") {
                score += candidate.tool_score * 0.10;
            }

            if score > best_score {
                best_score = score;

                selected = Some(ArbitrationDecision {
                    selected_provider: candidate.provider.clone(),

                    final_score: score,

                    rationale: format!("{} selected for workload optimization", candidate.provider),
                });
            }
        }

        selected
    }
}
