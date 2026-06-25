use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceSignal {
    pub signal_id: String,

    pub domain: String,

    pub governance_entropy: f64,

    pub replay_decay: f64,

    pub ecosystem_fragmentation: f64,

    pub topology_instability: f64,

    pub survivability_decay: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapseDirective {
    pub signal_id: String,

    pub collapse_risk: bool,

    pub intervention_required: bool,

    pub replay_preservation_critical: bool,

    pub topology_stabilization_required: bool,

    pub civilization_quarantine: bool,

    pub resilience_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceState {
    pub civilization_resilience: f64,

    pub governance_stability: f64,

    pub replay_continuity: f64,

    pub civilization_survivable: bool,

    pub directives: Vec<CollapseDirective>,
}

pub struct CivilizationResilienceEngine;

impl CivilizationResilienceEngine {
    pub fn protect(signals: &[ResilienceSignal]) -> ResilienceState {
        let mut directives = Vec::new();

        let mut resilience = 0.0;

        let mut governance = 0.0;

        let mut replay = 0.0;

        for signal in signals {
            println!("[RESILIENCE] signal={}", signal.signal_id);

            let resilience_score = ((1.0 - signal.governance_entropy) * 0.25)
                + ((1.0 - signal.replay_decay) * 0.20)
                + ((1.0 - signal.ecosystem_fragmentation) * 0.20)
                + ((1.0 - signal.topology_instability) * 0.20)
                + ((1.0 - signal.survivability_decay) * 0.15);

            let collapse_risk = resilience_score < 0.68;

            let intervention_required = resilience_score < 0.80;

            let replay_preservation_critical = signal.replay_decay > 0.70;

            let topology_stabilization_required = signal.topology_instability > 0.72;

            let civilization_quarantine = signal.ecosystem_fragmentation > 0.84;

            directives.push(CollapseDirective {
                signal_id: signal.signal_id.clone(),

                collapse_risk,

                intervention_required,

                replay_preservation_critical,

                topology_stabilization_required,

                civilization_quarantine,

                resilience_score,
            });

            resilience += resilience_score;

            governance += (1.0 - signal.governance_entropy);

            replay += (1.0 - signal.replay_decay);
        }

        let count = signals.len() as f64;

        let civilization_resilience = resilience / count;

        let governance_stability = governance / count;

        let replay_continuity = replay / count;

        let civilization_survivable = civilization_resilience > 0.82
            && governance_stability > 0.80
            && replay_continuity > 0.81;

        ResilienceState {
            civilization_resilience,

            governance_stability,

            replay_continuity,

            civilization_survivable,

            directives,
        }
    }
}
