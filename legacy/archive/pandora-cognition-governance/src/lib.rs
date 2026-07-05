//! Pandora Cognition Governance — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveMemory {
    pub memory_id: String,

    pub survivability: f64,

    pub relevance: f64,

    pub mutation_risk: f64,

    pub token_weight: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceDecision {
    pub decision_id: String,

    pub memory_id: String,

    pub action: String,

    pub target_mutation: String,

    pub reviewed_by: String,

    pub verdict: String,

    pub reasoning: String,

    pub timestamp: String,

    pub governance_score: f64,
}

pub struct CognitionPersistenceGovernance;

impl CognitionPersistenceGovernance {
    pub fn govern(memories: &[CognitiveMemory]) -> Vec<GovernanceDecision> {
        let mut decisions = Vec::new();

        for memory in memories {
            println!("[GOVERNANCE] evaluating {}", memory.memory_id);

            let score = (memory.survivability * 0.40) + (memory.relevance * 0.35)
                - (memory.mutation_risk * 0.25);

            let action = if score > 0.85 {
                "persist-active"
            } else if score > 0.65 {
                "archive-context"
            } else if memory.mutation_risk > 0.80 {
                "quarantine-memory"
            } else {
                "purge-memory"
            };

            decisions.push(GovernanceDecision {
                decision_id: format!("gov-{}-{:x}", memory.memory_id, 0u64),

                memory_id: memory.memory_id.clone(),

                action: action.into(),

                target_mutation: String::new(),

                reviewed_by: String::new(),

                verdict: action.into(),

                reasoning: String::new(),

                timestamp: String::new(),

                governance_score: score,
            });
        }

        decisions.sort_by(|a, b| b.governance_score.partial_cmp(&a.governance_score).unwrap());

        decisions
    }
}
