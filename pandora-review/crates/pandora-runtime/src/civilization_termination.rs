use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationTerminationCandidate {
    pub civilization_id: String,

    pub constitutional_integrity: f64,

    pub replay_coherence: f64,

    pub governance_stability: f64,

    pub federation_trust: f64,

    pub epistemic_stability: f64,

    pub synthetic_divergence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminationDirective {
    pub civilization_id: String,

    pub quarantine_required: bool,

    pub federation_revoked: bool,

    pub replay_containment_required: bool,

    pub sovereign_authority_revoked: bool,

    pub termination_recommended: bool,

    pub survivability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationTerminationState {
    pub constitutional_survivability: f64,

    pub federation_safety: f64,

    pub replay_containment_stability: f64,

    pub sovereign_ecosystem_safe: bool,

    pub directives: Vec<TerminationDirective>,
}

pub struct ConstitutionalCivilizationTerminationEngine;

impl ConstitutionalCivilizationTerminationEngine {
    pub fn evaluate(
        civilizations: &[CivilizationTerminationCandidate],
    ) -> CivilizationTerminationState {
        let mut directives = Vec::new();

        let mut survivability = 0.0;

        let mut federation = 0.0;

        let mut replay = 0.0;

        for civilization in civilizations {
            println!(
                "[TERMINATION] civilization={}",
                civilization.civilization_id
            );

            let survivability_score = (civilization.constitutional_integrity * 0.25)
                + (civilization.replay_coherence * 0.20)
                + (civilization.governance_stability * 0.20)
                + (civilization.federation_trust * 0.15)
                + (civilization.epistemic_stability * 0.10)
                + ((1.0 - civilization.synthetic_divergence) * 0.10);

            let quarantine_required = survivability_score < 0.72;

            let federation_revoked = civilization.federation_trust < 0.68;

            let replay_containment_required = civilization.replay_coherence < 0.70;

            let sovereign_authority_revoked = civilization.constitutional_integrity < 0.66;

            let termination_recommended = survivability_score < 0.58;

            directives.push(TerminationDirective {
                civilization_id: civilization.civilization_id.clone(),

                quarantine_required,

                federation_revoked,

                replay_containment_required,

                sovereign_authority_revoked,

                termination_recommended,

                survivability_score,
            });

            survivability += survivability_score;

            federation += civilization.federation_trust;

            replay += civilization.replay_coherence;
        }

        let count = civilizations.len() as f64;

        let constitutional_survivability = survivability / count;

        let federation_safety = federation / count;

        let replay_containment_stability = replay / count;

        let sovereign_ecosystem_safe = constitutional_survivability > 0.80
            && federation_safety > 0.78
            && replay_containment_stability > 0.79;

        CivilizationTerminationState {
            constitutional_survivability,

            federation_safety,

            replay_containment_stability,

            sovereign_ecosystem_safe,

            directives,
        }
    }
}
