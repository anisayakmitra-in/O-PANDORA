//! Absorbed from pandora-execution-ranking (Phase 1C).
//!
//! Pandora Execution Ranking — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCandidate {
    pub candidate_id: String,

    pub benchmark_score: f64,

    pub repair_success_rate: f64,

    pub stability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedExecution {
    pub candidate_id: String,

    pub total_score: f64,

    pub rank: usize,
}

pub struct ExecutionRankingEngine;

impl ExecutionRankingEngine {
    pub fn rank(candidates: &[ExecutionCandidate]) -> Vec<RankedExecution> {
        let mut ranked = Vec::new();

        for candidate in candidates {
            println!("[RANKING] evaluating {}", candidate.candidate_id);

            let score = (candidate.benchmark_score * 0.5)
                + (candidate.repair_success_rate * 0.3)
                + (candidate.stability_score * 0.2);

            ranked.push(RankedExecution {
                candidate_id: candidate.candidate_id.clone(),

                total_score: score,

                rank: 0,
            });
        }

        ranked.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());

        for (index, candidate) in ranked.iter_mut().enumerate() {
            candidate.rank = index + 1;
        }

        ranked
    }
}
