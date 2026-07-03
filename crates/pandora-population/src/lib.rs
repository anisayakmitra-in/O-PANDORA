//! Pandora Population — extracted from pandora-runtime (Phase 1B).
//!
use serde::{Deserialize, Serialize};

use crate::fitness::FitnessEvaluation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionCandidate {
    pub candidate_id: String,

    pub generation: u32,

    pub mutation_source: String,

    pub fitness: Option<FitnessEvaluation>,
}

pub struct PopulationManager {
    pub population: Vec<EvolutionCandidate>,
}

impl Default for PopulationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PopulationManager {
    pub fn new() -> Self {
        Self {
            population: Vec::new(),
        }
    }

    pub fn add_candidate(&mut self, candidate: EvolutionCandidate) {
        self.population.push(candidate);
    }

    pub fn best_candidate(&self) -> Option<&EvolutionCandidate> {
        self.population.iter().max_by(|a, b| {
            let a_score = a.fitness.as_ref().map(|f| f.final_score).unwrap_or(0.0);

            let b_score = b.fitness.as_ref().map(|f| f.final_score).unwrap_or(0.0);

            a_score.partial_cmp(&b_score).unwrap()
        })
    }
}
