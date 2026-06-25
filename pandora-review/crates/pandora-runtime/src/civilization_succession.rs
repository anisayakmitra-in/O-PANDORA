use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationSuccessor {
    pub civilization_id: String,

    pub replay_legitimacy: f64,

    pub constitutional_inheritance: f64,

    pub lineage_continuity: f64,

    pub federation_trust: f64,

    pub survivability_authority: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessionDirective {
    pub civilization_id: String,

    pub sovereign_successor: bool,

    pub replay_legitimacy_verified: bool,

    pub constitutional_authority_confirmed: bool,

    pub federation_inheritance_allowed: bool,

    pub succession_dispute_detected: bool,

    pub succession_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationSuccessionState {
    pub constitutional_succession_integrity: f64,

    pub replay_inheritance_stability: f64,

    pub sovereign_authority_coherence: f64,

    pub sovereign_succession_stable: bool,

    pub directives: Vec<SuccessionDirective>,
}

pub struct ConstitutionalCivilizationSuccessionEngine;

impl ConstitutionalCivilizationSuccessionEngine {
    pub fn arbitrate(civilizations: &[CivilizationSuccessor]) -> CivilizationSuccessionState {
        let mut directives = Vec::new();

        let mut succession = 0.0;

        let mut replay = 0.0;

        let mut authority = 0.0;

        for civilization in civilizations {
            println!("[SUCCESSION] civilization={}", civilization.civilization_id);

            let succession_score = (civilization.replay_legitimacy * 0.25)
                + (civilization.constitutional_inheritance * 0.25)
                + (civilization.lineage_continuity * 0.20)
                + (civilization.federation_trust * 0.15)
                + (civilization.survivability_authority * 0.15);

            let sovereign_successor = succession_score > 0.88;

            let replay_legitimacy_verified = civilization.replay_legitimacy > 0.86;

            let constitutional_authority_confirmed = civilization.constitutional_inheritance > 0.84;

            let federation_inheritance_allowed = civilization.federation_trust > 0.82;

            let succession_dispute_detected = succession_score < 0.72;

            directives.push(SuccessionDirective {
                civilization_id: civilization.civilization_id.clone(),

                sovereign_successor,

                replay_legitimacy_verified,

                constitutional_authority_confirmed,

                federation_inheritance_allowed,

                succession_dispute_detected,

                succession_score,
            });

            succession += succession_score;

            replay += civilization.replay_legitimacy;

            authority += civilization.constitutional_inheritance;
        }

        let count = civilizations.len() as f64;

        let constitutional_succession_integrity = succession / count;

        let replay_inheritance_stability = replay / count;

        let sovereign_authority_coherence = authority / count;

        let sovereign_succession_stable = constitutional_succession_integrity > 0.85
            && replay_inheritance_stability > 0.84
            && sovereign_authority_coherence > 0.85;

        CivilizationSuccessionState {
            constitutional_succession_integrity,

            replay_inheritance_stability,

            sovereign_authority_coherence,

            sovereign_succession_stable,

            directives,
        }
    }
}
