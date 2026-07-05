//! Pandora Evolution Parliament — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentChamber {
    pub chamber_id: String,

    pub domain: String,

    pub governance_weight: f64,

    pub survivability_bias: f64,

    pub replay_requirements: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProposal {
    pub proposal_id: String,

    pub domain: String,

    pub mutation_risk: f64,

    pub survivability_projection: f64,

    pub replay_integrity: f64,

    pub ecosystem_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentVerdict {
    pub proposal_id: String,

    pub constitutional_approved: bool,

    pub override_required: bool,

    pub survivability_consensus: f64,

    pub governance_consensus: f64,

    pub promotion_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentState {
    pub constitutional_stability: f64,

    pub survivability_alignment: f64,

    pub governance_alignment: f64,

    pub sovereign_deliberation_ready: bool,

    pub verdicts: Vec<ParliamentVerdict>,
}

pub struct ConstitutionalEvolutionParliament;

impl ConstitutionalEvolutionParliament {
    pub fn deliberate(
        chambers: &[ParliamentChamber],

        proposals: &[EvolutionProposal],
    ) -> ParliamentState {
        let mut verdicts = Vec::new();

        let mut constitutional = 0.0;

        let mut survivability = 0.0;

        let mut governance = 0.0;

        for proposal in proposals {
            println!("[PARLIAMENT] proposal={}", proposal.proposal_id);

            let relevant = chambers
                .iter()
                .filter(|chamber| chamber.domain == proposal.domain)
                .collect::<Vec<_>>();

            let governance_consensus = relevant
                .iter()
                .map(|chamber| chamber.governance_weight)
                .sum::<f64>()
                / relevant.len() as f64;

            let survivability_consensus = (proposal.survivability_projection * 0.50)
                + (proposal.replay_integrity * 0.30)
                + ((1.0 - proposal.mutation_risk) * 0.20);

            let constitutional_approved =
                governance_consensus > 0.82 && survivability_consensus > 0.84;

            let override_required = proposal.mutation_risk > 0.90;

            let promotion_tier = if constitutional_approved {
                "constitutional"
            } else if survivability_consensus > 0.72 {
                "restricted"
            } else {
                "quarantined"
            };

            verdicts.push(ParliamentVerdict {
                proposal_id: proposal.proposal_id.clone(),

                constitutional_approved,

                override_required,

                survivability_consensus,

                governance_consensus,

                promotion_tier: promotion_tier.into(),
            });

            constitutional += if constitutional_approved { 1.0 } else { 0.0 };

            survivability += survivability_consensus;

            governance += governance_consensus;
        }

        let count = proposals.len() as f64;

        let constitutional_stability = constitutional / count;

        let survivability_alignment = survivability / count;

        let governance_alignment = governance / count;

        let sovereign_deliberation_ready = constitutional_stability > 0.74
            && survivability_alignment > 0.82
            && governance_alignment > 0.81;

        ParliamentState {
            constitutional_stability,

            survivability_alignment,

            governance_alignment,

            sovereign_deliberation_ready,

            verdicts,
        }
    }
}
