//! Shadow Council.
//!
//! Constitutional approval runtime.
//! Every proposal flows through Shadow Council -> PANOPTES
//! -> Human Approval -> Apply -> Checkpoint -> Benchmark
//! -> Rollback if needed.

use pandora_types::universal::CouncilAction;
use serde::{Deserialize, Serialize};

/// Council proposal status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProposalStatus {
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Deferred,
    Applied,
    RolledBack,
}

/// Council proposal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CouncilProposal {
    pub proposal_id: String,
    pub subject: String,
    pub action: CouncilAction,
    pub rationale: String,
    pub status: ProposalStatus,
    pub trust_score: f64,
    pub requires_human_approval: bool,
    pub timestamp_ms: u64,
}

/// Helper to avoid cross-module CouncilAction imports.
pub fn council_action_approve() -> CouncilAction {
    CouncilAction::Approve
}

/// Shadow Council runtime.
pub struct ShadowCouncil;

impl ShadowCouncil {
    pub fn new() -> Self {
        ShadowCouncil
    }

    pub fn submit(&self, subject: &str, action: CouncilAction, rationale: &str) -> CouncilProposal {
        CouncilProposal {
            proposal_id: format!("proposal-{}", subject),
            subject: subject.to_string(),
            action,
            rationale: rationale.to_string(),
            status: ProposalStatus::Submitted,
            trust_score: 1.0,
            requires_human_approval: false,
            timestamp_ms: 0,
        }
    }

    pub fn approve(&self, proposal: &mut CouncilProposal) {
        proposal.status = ProposalStatus::Approved;
    }

    pub fn reject(&self, proposal: &mut CouncilProposal) {
        proposal.status = ProposalStatus::Rejected;
    }
}

impl Default for ShadowCouncil {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn council_proposal_lifecycle() {
        let c = ShadowCouncil::new();
        let mut p = c.submit("gene-1", CouncilAction::Approve, "benchmark ok");
        assert_eq!(p.status, ProposalStatus::Submitted);
        c.approve(&mut p);
        assert_eq!(p.status, ProposalStatus::Approved);
    }
}
