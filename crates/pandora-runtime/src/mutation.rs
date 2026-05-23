use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationProposal {
    pub mutation_id: String,

    pub target_gene: String,

    pub mutation_type: String,

    pub reason: String,

    pub proposed_by: String,

    pub lineage_parent: String,

    pub timestamp: String,
}
