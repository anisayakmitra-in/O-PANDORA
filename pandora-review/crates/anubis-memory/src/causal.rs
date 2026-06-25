use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    pub link_id: String,

    pub source_memory: String,

    pub target_memory: String,

    pub causal_reason: String,
}
