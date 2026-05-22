use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub enum GovernanceVerdict {

    Approved,

    Rejected,

    Escalated,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct GovernanceDecision {

    pub decision_id:
        String,

    pub target_mutation:
        String,

    pub reviewed_by:
        String,

    pub verdict:
        GovernanceVerdict,

    pub reasoning:
        String,

    pub timestamp:
        String,
}
