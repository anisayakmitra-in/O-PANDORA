use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationProposal {
    pub agent_id: String,

    pub task_id: String,

    pub confidence: f32,
}

pub struct SwarmNegotiator;

impl SwarmNegotiator {
    pub fn negotiate(proposals: &[NegotiationProposal]) -> Option<NegotiationProposal> {
        let selected = proposals
            .iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
            .cloned();

        if let Some(winner) = &selected {
            println!(
                "[NEGOTIATION] {} won task {}",
                winner.agent_id, winner.task_id
            );
        }

        selected
    }
}
