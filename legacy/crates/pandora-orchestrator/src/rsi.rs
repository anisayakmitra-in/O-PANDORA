use crate::gepa::{GepaObserver, MutationCandidate, MutationTarget};
use anyhow::{anyhow, Result};
use pandora_types::session::Session;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RsiStage {
    Proposed,
    AwaitingApproval,
    Approved,
    Applied,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RsiProposal {
    pub id: String,
    pub mutation_id: String,
    pub target_kind: MutationTarget,
    pub target_id: String,
    pub rationale: String,
    pub confidence: f32,
    pub stage: RsiStage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DsrRequest {
    pub proposal_id: String,
    pub old_implementation: String,
    pub new_implementation: String,
    pub new_package_hash: String,
    pub rollback_target: String,
    pub approval_id: Option<String>,
    pub verified: bool,
}

impl RsiProposal {
    fn from_candidate(candidate: MutationCandidate) -> Self {
        Self {
            id: format!("rsi-{}", candidate.id),
            mutation_id: candidate.id,
            target_kind: candidate.target_kind,
            target_id: candidate.target_id,
            rationale: candidate.proposal,
            confidence: candidate.confidence,
            stage: RsiStage::AwaitingApproval,
        }
    }
}

pub struct RsiCoordinator<'a> {
    observer: &'a GepaObserver,
}

impl<'a> RsiCoordinator<'a> {
    pub fn new(observer: &'a GepaObserver) -> Self {
        Self { observer }
    }

    pub fn propose(&self, session: &Session) -> Vec<RsiProposal> {
        self.observer
            .observe(session)
            .into_iter()
            .map(RsiProposal::from_candidate)
            .collect()
    }

    pub fn prepare_dsr(
        &self,
        proposal: &RsiProposal,
        old_implementation: &str,
        new_implementation: &str,
        new_package_hash: &str,
        rollback_target: &str,
        approval_id: Option<String>,
    ) -> Result<DsrRequest> {
        if proposal.stage != RsiStage::Approved {
            return Err(anyhow!("DSR requires an approved RSI proposal"));
        }
        if old_implementation.is_empty()
            || new_implementation.is_empty()
            || new_package_hash.is_empty()
            || rollback_target.is_empty()
        {
            return Err(anyhow!(
                "DSR requires implementation, hash, and rollback metadata"
            ));
        }
        if approval_id.is_none() {
            return Err(anyhow!("DSR requires a recorded approval"));
        }
        Ok(DsrRequest {
            proposal_id: proposal.id.clone(),
            old_implementation: old_implementation.to_string(),
            new_implementation: new_implementation.to_string(),
            new_package_hash: new_package_hash.to_string(),
            rollback_target: rollback_target.to_string(),
            approval_id,
            verified: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsr_requires_approval_and_rollback_metadata() {
        let observer =
            GepaObserver::new(std::env::temp_dir().join(format!("rsi-{}", rand::random::<u64>())));
        let coordinator = RsiCoordinator::new(&observer);
        let proposal = RsiProposal {
            id: "rsi-1".into(),
            mutation_id: "mutation-1".into(),
            target_kind: MutationTarget::Gene,
            target_id: "gene".into(),
            rationale: "improve".into(),
            confidence: 0.8,
            stage: RsiStage::Approved,
        };
        assert!(coordinator
            .prepare_dsr(
                &proposal,
                "old",
                "new",
                "sha256:abc",
                "old",
                Some("approval".into())
            )
            .is_ok());
        assert!(coordinator
            .prepare_dsr(&proposal, "old", "new", "", "old", Some("approval".into()))
            .is_err());
    }
}
