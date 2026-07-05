//! Absorbed from pandora-tournament (Phase 1C), with fixes.
//!
use crate::evolution::EvolutionCandidate;

pub struct TournamentSelector;

impl TournamentSelector {
    pub fn select(candidates: &[EvolutionCandidate]) -> Option<EvolutionCandidate> {
        candidates.iter().cloned().max_by(|a, b| {
            a.fitness
                .partial_cmp(&b.fitness)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}
