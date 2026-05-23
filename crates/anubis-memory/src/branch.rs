use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionBranch {
    pub branch_id: String,

    pub parent_branch: Option<String>,

    pub originating_memory: String,

    pub branch_reason: String,

    pub speculative: bool,
}
