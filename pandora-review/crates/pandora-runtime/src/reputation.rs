use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationNode {
    pub node_id: String,

    pub reputation: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationVote {
    pub node_id: String,

    pub accepted: bool,
}

pub struct ReputationConsensus;

impl ReputationConsensus {
    pub fn evaluate(nodes: &[ReputationNode], votes: &[ReputationVote]) -> bool {
        let mut accepted_weight = 0.0;

        let mut rejected_weight = 0.0;

        for vote in votes {
            let reputation = nodes
                .iter()
                .find(|n| n.node_id == vote.node_id)
                .map(|n| n.reputation)
                .unwrap_or(0.0);

            if vote.accepted {
                accepted_weight += reputation;
            } else {
                rejected_weight += reputation;
            }
        }

        println!(
            "[REPUTATION] accepted_weight={} rejected_weight={}",
            accepted_weight, rejected_weight
        );

        accepted_weight > rejected_weight
    }
}
