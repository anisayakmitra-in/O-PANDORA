use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegenerationSignal {
    pub signal_id: String,

    pub domain: String,

    pub governance_damage: f64,

    pub replay_loss: f64,

    pub topology_decay: f64,

    pub ecosystem_fragmentation: f64,

    pub survivability_loss: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegenerationDirective {
    pub signal_id: String,

    pub regeneration_required: bool,

    pub governance_reconstruction: bool,

    pub replay_restoration: bool,

    pub topology_reconstruction: bool,

    pub ecosystem_reintegration: bool,

    pub regeneration_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegenerationState {
    pub civilization_regeneration_capacity: f64,

    pub governance_recovery_alignment: f64,

    pub replay_restoration_integrity: f64,

    pub sovereign_regeneration_viable: bool,

    pub directives: Vec<RegenerationDirective>,
}

pub struct CivilizationRegenerationEngine;

impl CivilizationRegenerationEngine {
    pub fn regenerate(signals: &[RegenerationSignal]) -> RegenerationState {
        let mut directives = Vec::new();

        let mut regeneration = 0.0;

        let mut governance = 0.0;

        let mut replay = 0.0;

        for signal in signals {
            println!("[REGENERATION] signal={}", signal.signal_id);

            let regeneration_score = ((1.0 - signal.governance_damage) * 0.25)
                + ((1.0 - signal.replay_loss) * 0.20)
                + ((1.0 - signal.topology_decay) * 0.20)
                + ((1.0 - signal.ecosystem_fragmentation) * 0.20)
                + ((1.0 - signal.survivability_loss) * 0.15);

            let regeneration_required = regeneration_score < 0.84;

            let governance_reconstruction = signal.governance_damage > 0.45;

            let replay_restoration = signal.replay_loss > 0.38;

            let topology_reconstruction = signal.topology_decay > 0.42;

            let ecosystem_reintegration = signal.ecosystem_fragmentation > 0.40;

            directives.push(RegenerationDirective {
                signal_id: signal.signal_id.clone(),

                regeneration_required,

                governance_reconstruction,

                replay_restoration,

                topology_reconstruction,

                ecosystem_reintegration,

                regeneration_score,
            });

            regeneration += regeneration_score;

            governance += 1.0 - signal.governance_damage;

            replay += 1.0 - signal.replay_loss;
        }

        let count = signals.len() as f64;

        let civilization_regeneration_capacity = regeneration / count;

        let governance_recovery_alignment = governance / count;

        let replay_restoration_integrity = replay / count;

        let sovereign_regeneration_viable = civilization_regeneration_capacity > 0.80
            && governance_recovery_alignment > 0.79
            && replay_restoration_integrity > 0.81;

        RegenerationState {
            civilization_regeneration_capacity,

            governance_recovery_alignment,

            replay_restoration_integrity,

            sovereign_regeneration_viable,

            directives,
        }
    }
}
