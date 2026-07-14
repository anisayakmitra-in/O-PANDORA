//! Pandora Uncertainty Topology — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintySignal {
    pub signal_id: String,

    pub domain: String,

    pub simulation_divergence: f64,

    pub governance_ambiguity: f64,

    pub topology_instability: f64,

    pub replay_confidence_loss: f64,

    pub survivability_uncertainty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyDirective {
    pub signal_id: String,

    pub uncertainty_zone: bool,

    pub constitutional_intervention: bool,

    pub autonomy_constraint_required: bool,

    pub replay_verification_required: bool,

    pub topology_reassessment_required: bool,

    pub uncertainty_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyState {
    pub civilization_certainty: f64,

    pub governance_clarity: f64,

    pub replay_confidence: f64,

    pub sovereign_uncertainty_stable: bool,

    pub directives: Vec<UncertaintyDirective>,
}

pub struct UncertaintyTopologyEngine;

impl UncertaintyTopologyEngine {
    pub fn map(signals: &[UncertaintySignal]) -> UncertaintyState {
        let mut directives = Vec::new();

        let mut certainty = 0.0;

        let mut governance = 0.0;

        let mut replay = 0.0;

        for signal in signals {
            println!("[UNCERTAINTY] signal={}", signal.signal_id);

            let uncertainty_score = (signal.simulation_divergence * 0.25)
                + (signal.governance_ambiguity * 0.20)
                + (signal.topology_instability * 0.20)
                + (signal.replay_confidence_loss * 0.15)
                + (signal.survivability_uncertainty * 0.20);

            let uncertainty_zone = uncertainty_score > 0.62;

            let constitutional_intervention = uncertainty_score > 0.74;

            let autonomy_constraint_required = signal.simulation_divergence > 0.68;

            let replay_verification_required = signal.replay_confidence_loss > 0.52;

            let topology_reassessment_required = signal.topology_instability > 0.66;

            directives.push(UncertaintyDirective {
                signal_id: signal.signal_id.clone(),

                uncertainty_zone,

                constitutional_intervention,

                autonomy_constraint_required,

                replay_verification_required,

                topology_reassessment_required,

                uncertainty_score,
            });

            certainty += 1.0 - uncertainty_score;

            governance += 1.0 - signal.governance_ambiguity;

            replay += 1.0 - signal.replay_confidence_loss;
        }

        let count = signals.len() as f64;

        let civilization_certainty = certainty / count;

        let governance_clarity = governance / count;

        let replay_confidence = replay / count;

        let sovereign_uncertainty_stable =
            civilization_certainty > 0.78 && governance_clarity > 0.80 && replay_confidence > 0.81;

        UncertaintyState {
            civilization_certainty,

            governance_clarity,

            replay_confidence,

            sovereign_uncertainty_stable,

            directives,
        }
    }
}
