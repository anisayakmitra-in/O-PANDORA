//! Tournament — consolidated into pandora-coordination.
//!
use crate::population::EvolutionCandidate;

pub struct TournamentSelector;

impl TournamentSelector {
    pub fn select(candidates: &[EvolutionCandidate]) -> Option<EvolutionCandidate> {
        candidates.iter().cloned().max_by(|a, b| {
            let a_score = a.fitness.as_ref().map(|f| f.final_score).unwrap_or(0.0);

            let b_score = b.fitness.as_ref().map(|f| f.final_score).unwrap_or(0.0);

            a_score.partial_cmp(&b_score).unwrap()
        })
    }
}
