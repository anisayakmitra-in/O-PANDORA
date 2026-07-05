//! Pandora Mutation — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationProposal {
    pub mutation_id: String,

    pub domain: String,

    pub lineage_depth: usize,

    pub governance_risk: f64,

    pub compatibility_score: f64,

    pub survivability_projection: f64,
}
