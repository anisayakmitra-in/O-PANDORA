use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationGenesisCandidate {
    pub civilization_id: String,

    pub provenance_integrity: f64,

    pub constitutional_foundation: f64,

    pub replay_seed_validity: f64,

    pub governance_initialization: f64,

    pub survivability_projection: f64,

    pub synthetic_origin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisDirective {
    pub civilization_id: String,

    pub sovereign_genesis_approved: bool,

    pub replay_seed_verified: bool,

    pub constitutional_foundation_valid: bool,

    pub federation_admission_allowed: bool,

    pub synthetic_quarantine_required: bool,

    pub genesis_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationGenesisState {
    pub constitutional_genesis_integrity: f64,

    pub replay_origin_stability: f64,

    pub sovereign_foundation_coherence: f64,

    pub sovereign_genesis_stable: bool,

    pub directives: Vec<GenesisDirective>,
}

pub struct ConstitutionalCivilizationGenesisEngine;

impl ConstitutionalCivilizationGenesisEngine {
    pub fn authorize(candidates: &[CivilizationGenesisCandidate]) -> CivilizationGenesisState {
        let mut directives = Vec::new();

        let mut integrity = 0.0;

        let mut replay = 0.0;

        let mut foundation = 0.0;

        for candidate in candidates {
            println!("[GENESIS] civilization={}", candidate.civilization_id);

            let genesis_score = (candidate.provenance_integrity * 0.20)
                + (candidate.constitutional_foundation * 0.25)
                + (candidate.replay_seed_validity * 0.20)
                + (candidate.governance_initialization * 0.15)
                + (candidate.survivability_projection * 0.20);

            let sovereign_genesis_approved = genesis_score > 0.86;

            let replay_seed_verified = candidate.replay_seed_validity > 0.84;

            let constitutional_foundation_valid = candidate.constitutional_foundation > 0.85;

            let federation_admission_allowed = genesis_score > 0.82;

            let synthetic_quarantine_required = candidate.synthetic_origin && genesis_score < 0.90;

            directives.push(GenesisDirective {
                civilization_id: candidate.civilization_id.clone(),

                sovereign_genesis_approved,

                replay_seed_verified,

                constitutional_foundation_valid,

                federation_admission_allowed,

                synthetic_quarantine_required,

                genesis_score,
            });

            integrity += genesis_score;

            replay += candidate.replay_seed_validity;

            foundation += candidate.constitutional_foundation;
        }

        let count = candidates.len() as f64;

        let constitutional_genesis_integrity = integrity / count;

        let replay_origin_stability = replay / count;

        let sovereign_foundation_coherence = foundation / count;

        let sovereign_genesis_stable = constitutional_genesis_integrity > 0.85
            && replay_origin_stability > 0.84
            && sovereign_foundation_coherence > 0.85;

        CivilizationGenesisState {
            constitutional_genesis_integrity,

            replay_origin_stability,

            sovereign_foundation_coherence,

            sovereign_genesis_stable,

            directives,
        }
    }
}
