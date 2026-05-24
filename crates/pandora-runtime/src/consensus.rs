use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusVote {
    pub node_id: String,

    pub proposal: String,

    pub accepted: bool,
}

pub struct ConsensusCoordinator;

impl ConsensusCoordinator {
    pub fn evaluate(proposal: &str, votes: &[ConsensusVote]) -> bool {
        let accepted = votes.iter().filter(|v| v.accepted).count();

        let rejected = votes.len() - accepted;

        println!(
            "[CONSENSUS] proposal={} accepted={} rejected={}",
            proposal, accepted, rejected
        );

        accepted > rejected
    }
}
