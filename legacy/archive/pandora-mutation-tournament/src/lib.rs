//! Pandora Mutation Tournament — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationCandidate {
    pub id: String,

    pub benchmark_score: f64,

    pub repair_score: f64,

    pub survivability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentWinner {
    pub id: String,

    pub evolutionary_score: f64,
}

pub struct MutationTournamentEngine;

impl MutationTournamentEngine {
    pub fn compete(candidates: &[MutationCandidate]) -> Option<TournamentWinner> {
        let mut best_score = 0.0;

        let mut winner = None;

        for candidate in candidates {
            println!("[TOURNAMENT] evaluating {}", candidate.id);

            let score = (candidate.benchmark_score * 0.4)
                + (candidate.repair_score * 0.3)
                + (candidate.survivability_score * 0.3);

            println!("[TOURNAMENT] score={}", score);

            if score > best_score {
                best_score = score;

                winner = Some(TournamentWinner {
                    id: candidate.id.clone(),

                    evolutionary_score: score,
                });
            }
        }

        winner
    }
}
