use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropySignal {
    pub signal_id: String,

    pub domain: String,

    pub recursive_entropy: f64,

    pub governance_drift: f64,

    pub replay_fragmentation: f64,

    pub mutation_instability: f64,

    pub autonomy_degradation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapseDirective {
    pub signal_id: String,

    pub entropy_collapse_detected: bool,

    pub governance_intervention: bool,

    pub replay_reconstruction_required: bool,

    pub mutation_freeze_required: bool,

    pub autonomy_constraint_required: bool,

    pub collapse_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapseState {
    pub constitutional_stability: f64,

    pub replay_integrity: f64,

    pub governance_coherence: f64,

    pub sovereign_collapse_risk: bool,

    pub directives: Vec<CollapseDirective>,
}

pub struct EntropyCollapseEngine;

impl EntropyCollapseEngine {
    pub fn analyze(signals: &[EntropySignal]) -> CollapseState {
        let mut directives = Vec::new();

        let mut stability = 0.0;

        let mut replay = 0.0;

        let mut governance = 0.0;

        for signal in signals {
            println!("[COLLAPSE] signal={}", signal.signal_id);

            let collapse_score = (signal.recursive_entropy * 0.25)
                + (signal.governance_drift * 0.20)
                + (signal.replay_fragmentation * 0.20)
                + (signal.mutation_instability * 0.20)
                + (signal.autonomy_degradation * 0.15);

            let entropy_collapse_detected = collapse_score > 0.70;

            let governance_intervention = signal.governance_drift > 0.62;

            let replay_reconstruction_required = signal.replay_fragmentation > 0.58;

            let mutation_freeze_required = signal.mutation_instability > 0.66;

            let autonomy_constraint_required = signal.autonomy_degradation > 0.61;

            directives.push(CollapseDirective {
                signal_id: signal.signal_id.clone(),

                entropy_collapse_detected,

                governance_intervention,

                replay_reconstruction_required,

                mutation_freeze_required,

                autonomy_constraint_required,

                collapse_score,
            });

            stability += 1.0 - collapse_score;

            replay += 1.0 - signal.replay_fragmentation;

            governance += 1.0 - signal.governance_drift;
        }

        let count = signals.len() as f64;

        let constitutional_stability = stability / count;

        let replay_integrity = replay / count;

        let governance_coherence = governance / count;

        let sovereign_collapse_risk = constitutional_stability < 0.64
            || replay_integrity < 0.66
            || governance_coherence < 0.67;

        CollapseState {
            constitutional_stability,

            replay_integrity,

            governance_coherence,

            sovereign_collapse_risk,

            directives,
        }
    }
}
