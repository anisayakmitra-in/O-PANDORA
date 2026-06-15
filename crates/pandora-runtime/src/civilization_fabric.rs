use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationNode {
    pub civilization_id: String,

    pub governance_doctrine: String,

    pub replay_trust_score: f64,

    pub autonomy_alignment: f64,

    pub constitutional_compatibility: f64,

    pub synthetic_exchange_allowed: bool,

    pub survivability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationDirective {
    pub civilization_id: String,

    pub federation_allowed: bool,

    pub replay_federation_allowed: bool,

    pub autonomy_interoperable: bool,

    pub synthetic_exchange_authorized: bool,

    pub constitutional_quarantine: bool,

    pub federation_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationFabricState {
    pub federation_integrity: f64,

    pub replay_federation_stability: f64,

    pub constitutional_alignment: f64,

    pub sovereign_fabric_stable: bool,

    pub directives: Vec<FederationDirective>,
}

pub struct ConstitutionalCivilizationFabricEngine;

impl ConstitutionalCivilizationFabricEngine {
    pub fn federate(civilizations: &[CivilizationNode]) -> CivilizationFabricState {
        let mut directives = Vec::new();

        let mut federation = 0.0;

        let mut replay = 0.0;

        let mut alignment = 0.0;

        for civilization in civilizations {
            println!("[FABRIC] civilization={}", civilization.civilization_id);

            let federation_score = (civilization.replay_trust_score * 0.20)
                + (civilization.autonomy_alignment * 0.20)
                + (civilization.constitutional_compatibility * 0.30)
                + (civilization.survivability_score * 0.20)
                + (if civilization.synthetic_exchange_allowed {
                    1.0
                } else {
                    0.0
                } * 0.10);

            let federation_allowed = federation_score > 0.82;

            let replay_federation_allowed = civilization.replay_trust_score > 0.84;

            let autonomy_interoperable = civilization.autonomy_alignment > 0.80;

            let synthetic_exchange_authorized = civilization.synthetic_exchange_allowed
                && civilization.constitutional_compatibility > 0.86;

            let constitutional_quarantine = civilization.constitutional_compatibility < 0.68;

            directives.push(FederationDirective {
                civilization_id: civilization.civilization_id.clone(),

                federation_allowed,

                replay_federation_allowed,

                autonomy_interoperable,

                synthetic_exchange_authorized,

                constitutional_quarantine,

                federation_score,
            });

            federation += federation_score;

            replay += civilization.replay_trust_score;

            alignment += civilization.constitutional_compatibility;
        }

        let count = civilizations.len() as f64;

        let federation_integrity = federation / count;

        let replay_federation_stability = replay / count;

        let constitutional_alignment = alignment / count;

        let sovereign_fabric_stable = federation_integrity > 0.84
            && replay_federation_stability > 0.83
            && constitutional_alignment > 0.84;

        CivilizationFabricState {
            federation_integrity,

            replay_federation_stability,

            constitutional_alignment,

            sovereign_fabric_stable,

            directives,
        }
    }
}
