use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationRebirthCandidate {
    pub civilization_id: String,

    pub replay_reconstruction: f64,

    pub constitutional_rehabilitation: f64,

    pub governance_stabilization: f64,

    pub federation_reacceptance: f64,

    pub epistemic_recovery: f64,

    pub synthetic_contamination_removed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebirthDirective {
    pub civilization_id: String,

    pub rebirth_authorized: bool,

    pub replay_reintegration_allowed: bool,

    pub sovereign_authority_restored: bool,

    pub federation_reentry_allowed: bool,

    pub rehabilitation_incomplete: bool,

    pub rebirth_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationRebirthState {
    pub constitutional_rehabilitation_integrity: f64,

    pub replay_recovery_stability: f64,

    pub federation_reintegration_coherence: f64,

    pub sovereign_rebirth_stable: bool,

    pub directives: Vec<RebirthDirective>,
}

pub struct ConstitutionalCivilizationRebirthEngine;

impl ConstitutionalCivilizationRebirthEngine {
    pub fn rehabilitate(
        civilizations: &[CivilizationRebirthCandidate],
    ) -> CivilizationRebirthState {
        let mut directives = Vec::new();

        let mut rehabilitation = 0.0;

        let mut replay = 0.0;

        let mut federation = 0.0;

        for civilization in civilizations {
            println!("[REBIRTH] civilization={}", civilization.civilization_id);

            let rebirth_score = (civilization.replay_reconstruction * 0.20)
                + (civilization.constitutional_rehabilitation * 0.25)
                + (civilization.governance_stabilization * 0.20)
                + (civilization.federation_reacceptance * 0.15)
                + (civilization.epistemic_recovery * 0.10)
                + (if civilization.synthetic_contamination_removed {
                    1.0
                } else {
                    0.0
                } * 0.10);

            let rebirth_authorized = rebirth_score > 0.84;

            let replay_reintegration_allowed = civilization.replay_reconstruction > 0.82;

            let sovereign_authority_restored = civilization.constitutional_rehabilitation > 0.86;

            let federation_reentry_allowed = civilization.federation_reacceptance > 0.80;

            let rehabilitation_incomplete = rebirth_score < 0.72;

            directives.push(RebirthDirective {
                civilization_id: civilization.civilization_id.clone(),

                rebirth_authorized,

                replay_reintegration_allowed,

                sovereign_authority_restored,

                federation_reentry_allowed,

                rehabilitation_incomplete,

                rebirth_score,
            });

            rehabilitation += rebirth_score;

            replay += civilization.replay_reconstruction;

            federation += civilization.federation_reacceptance;
        }

        let count = civilizations.len() as f64;

        let constitutional_rehabilitation_integrity = rehabilitation / count;

        let replay_recovery_stability = replay / count;

        let federation_reintegration_coherence = federation / count;

        let sovereign_rebirth_stable = constitutional_rehabilitation_integrity > 0.82
            && replay_recovery_stability > 0.80
            && federation_reintegration_coherence > 0.78;

        CivilizationRebirthState {
            constitutional_rehabilitation_integrity,

            replay_recovery_stability,

            federation_reintegration_coherence,

            sovereign_rebirth_stable,

            directives,
        }
    }
}
