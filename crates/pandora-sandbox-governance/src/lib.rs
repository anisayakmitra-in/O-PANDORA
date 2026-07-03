//! Pandora Sandbox Governance — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationProposal {
    pub mutation_id: String,

    pub domain: String,

    pub lineage_depth: usize,

    pub governance_risk: f64,

    pub compatibility_score: f64,

    pub survivability_projection: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxEnvironment {
    pub sandbox_id: String,

    pub isolation_strength: f64,

    pub telemetry_visibility: f64,

    pub replay_support: bool,

    pub benchmark_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceValidation {
    pub mutation_id: String,

    pub approved: bool,

    pub sandbox_required: bool,

    pub promotion_ready: bool,

    pub rollback_required: bool,

    pub oversight_required: bool,
}

pub struct SandboxGovernanceEngine;

impl SandboxGovernanceEngine {
    pub fn validate(
        proposals: &[MutationProposal],

        sandboxes: &[SandboxEnvironment],
    ) -> Vec<GovernanceValidation> {
        let mut validations = Vec::new();

        for proposal in proposals {
            println!("[SANDBOX] mutation={}", proposal.mutation_id);

            let sandbox = sandboxes.iter().max_by(|a, b| {
                let score_a = (a.isolation_strength * 0.45)
                    + (a.telemetry_visibility * 0.35)
                    + (if a.replay_support { 1.0 } else { 0.0 } * 0.20);

                let score_b = (b.isolation_strength * 0.45)
                    + (b.telemetry_visibility * 0.35)
                    + (if b.replay_support { 1.0 } else { 0.0 } * 0.20);

                score_a.partial_cmp(&score_b).unwrap()
            });

            let sandbox_required = proposal.governance_risk > 0.55;

            let promotion_ready = proposal.compatibility_score > 0.85
                && proposal.survivability_projection > 0.82
                && sandbox.map(|s| s.benchmark_ready).unwrap_or(false);

            let rollback_required = proposal.governance_risk > 0.88;

            let oversight_required = proposal.lineage_depth > 8;

            let approved = promotion_ready && !rollback_required;

            validations.push(GovernanceValidation {
                mutation_id: proposal.mutation_id.clone(),

                approved,

                sandbox_required,

                promotion_ready,

                rollback_required,

                oversight_required,
            });
        }

        validations
    }
}
