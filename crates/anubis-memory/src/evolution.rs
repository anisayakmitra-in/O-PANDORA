use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchScore {
    pub branch_id: String,

    pub fitness: f32,

    pub confidence: f32,

    pub governance_penalty: f32,

    pub mutation_depth: u32,
}

pub struct EvolutionarySelector;

impl EvolutionarySelector {
    pub fn select_best(scores: &[BranchScore]) -> Option<&BranchScore> {
        scores.iter().max_by(|a, b| {
            let score_a = a.fitness * a.confidence - a.governance_penalty;

            let score_b = b.fitness * b.confidence - b.governance_penalty;

            score_a.partial_cmp(&score_b).unwrap()
        })
    }

    pub fn top_k(scores: &[BranchScore], k: usize) -> Vec<&BranchScore> {
        let mut ranked = scores.iter().collect::<Vec<_>>();

        ranked.sort_by(|a, b| {
            let score_a = a.fitness * a.confidence - a.governance_penalty;

            let score_b = b.fitness * b.confidence - b.governance_penalty;

            score_b.partial_cmp(&score_a).unwrap()
        });

        ranked.into_iter().take(k).collect()
    }
}
