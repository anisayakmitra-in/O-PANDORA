use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalDoctrine {
    pub doctrine_id: String,

    pub domain: String,

    pub survivability_mandate: f64,

    pub governance_mandate: f64,

    pub replay_preservation: f64,

    pub autonomy_constraints: f64,

    pub civilization_priority: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalDirective {
    pub doctrine_id: String,

    pub constitutionally_valid: bool,

    pub governance_enforced: bool,

    pub replay_mandatory: bool,

    pub autonomy_restricted: bool,

    pub civilization_protected: bool,

    pub constitutional_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionState {
    pub sovereign_constitutional_integrity: f64,

    pub civilization_governance_stability: f64,

    pub replay_civilization_alignment: f64,

    pub sovereign_constitution_stable: bool,

    pub directives: Vec<ConstitutionalDirective>,
}

pub struct SovereignExecutionConstitution;

impl SovereignExecutionConstitution {
    pub fn govern(doctrines: &[ConstitutionalDoctrine]) -> ConstitutionState {
        let mut directives = Vec::new();

        let mut integrity = 0.0;

        let mut governance = 0.0;

        let mut replay = 0.0;

        for doctrine in doctrines {
            println!("[CONSTITUTION] doctrine={}", doctrine.doctrine_id);

            let constitutional_score = (doctrine.survivability_mandate * 0.25)
                + (doctrine.governance_mandate * 0.25)
                + (doctrine.replay_preservation * 0.20)
                + ((1.0 - doctrine.autonomy_constraints) * 0.10)
                + (doctrine.civilization_priority * 0.20);

            let constitutionally_valid = constitutional_score > 0.84;

            let governance_enforced = doctrine.governance_mandate > 0.82;

            let replay_mandatory = doctrine.replay_preservation > 0.85;

            let autonomy_restricted = doctrine.autonomy_constraints > 0.72;

            let civilization_protected = doctrine.civilization_priority > 0.88;

            directives.push(ConstitutionalDirective {
                doctrine_id: doctrine.doctrine_id.clone(),

                constitutionally_valid,

                governance_enforced,

                replay_mandatory,

                autonomy_restricted,

                civilization_protected,

                constitutional_score,
            });

            integrity += constitutional_score;

            governance += doctrine.governance_mandate;

            replay += doctrine.replay_preservation;
        }

        let count = doctrines.len() as f64;

        let sovereign_constitutional_integrity = integrity / count;

        let civilization_governance_stability = governance / count;

        let replay_civilization_alignment = replay / count;

        let sovereign_constitution_stable = sovereign_constitutional_integrity > 0.85
            && civilization_governance_stability > 0.83
            && replay_civilization_alignment > 0.84;

        ConstitutionState {
            sovereign_constitutional_integrity,

            civilization_governance_stability,

            replay_civilization_alignment,

            sovereign_constitution_stable,

            directives,
        }
    }
}
