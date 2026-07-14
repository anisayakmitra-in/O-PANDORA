//! Pandora Mutation Rollback — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRecord {
    pub mutation_id: String,

    pub approved: bool,

    pub replay_score: f32,

    pub reverted: bool,
}

pub struct MutationRollback;

impl MutationRollback {
    pub fn evaluate(mutation: &mut MutationRecord) {
        if mutation.replay_score < 0.5 {
            mutation.reverted = true;
        }
    }
}
