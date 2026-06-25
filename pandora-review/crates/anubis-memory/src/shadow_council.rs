use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilMember {
    pub member_id: String,

    pub harness_type: String,

    pub specialization: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliberationProposal {
    pub proposal_id: String,

    pub branch_id: String,

    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteDecision {
    Approve,

    Reject,

    Quarantine,

    Escalate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilVote {
    pub member_id: String,

    pub proposal_id: String,

    pub decision: VoteDecision,

    pub confidence: f32,
}

pub struct ShadowCouncil;

impl ShadowCouncil {
    pub fn consensus(votes: &[CouncilVote]) -> VoteDecision {
        let approvals = votes
            .iter()
            .filter(|vote| matches!(vote.decision, VoteDecision::Approve))
            .count();

        let rejections = votes
            .iter()
            .filter(|vote| matches!(vote.decision, VoteDecision::Reject))
            .count();

        if approvals >= rejections {
            VoteDecision::Approve
        } else {
            VoteDecision::Reject
        }
    }
}

impl ShadowCouncil {
    pub fn weighted_confidence(votes: &[CouncilVote]) -> f32 {
        if votes.is_empty() {
            return 0.0;
        }

        let total = votes.iter().map(|vote| vote.confidence).sum::<f32>();

        total / votes.len() as f32
    }
}
