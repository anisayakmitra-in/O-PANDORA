//! DSR replacement request validation.

use super::evolution::{EvolutionProposal, EvolutionStage};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplacementRequest {
    pub proposal_id: String,
    pub old_implementation: String,
    pub new_implementation: String,
    pub new_package_hash: String,
    pub rollback_target: String,
    pub approval_id: Option<String>,
    pub verified: bool,
}

pub struct ReplacementEngine;

impl ReplacementEngine {
    pub fn prepare(
        proposal: &EvolutionProposal,
        old_implementation: &str,
        new_implementation: &str,
        new_package_hash: &str,
        rollback_target: &str,
        approval_id: Option<String>,
    ) -> Result<ReplacementRequest> {
        if proposal.stage != EvolutionStage::Approved {
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
        Ok(ReplacementRequest {
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
    use crate::engines::mutation::MutationTarget;

    #[test]
    fn replacement_requires_approval_and_rollback_metadata() {
        let proposal = EvolutionProposal {
            id: "evolution-1".into(),
            mutation_id: "mutation-1".into(),
            target_kind: MutationTarget::Gene,
            target_id: "gene".into(),
            rationale: "improve".into(),
            confidence: 0.8,
            stage: EvolutionStage::Approved,
        };

        assert!(ReplacementEngine::prepare(
            &proposal,
            "old",
            "new",
            "sha256:abc",
            "old",
            Some("approval".into())
        )
        .is_ok());
        assert!(ReplacementEngine::prepare(
            &proposal,
            "old",
            "new",
            "",
            "old",
            Some("approval".into())
        )
        .is_err());
        assert!(ReplacementEngine::prepare(
            &proposal,
            "old",
            "new",
            "sha256:abc",
            "",
            Some("approval".into())
        )
        .is_err());
        assert!(
            ReplacementEngine::prepare(&proposal, "old", "new", "sha256:abc", "old", None).is_err()
        );
    }
}
