//! Pandora Swarm Metabolism — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicState {
    pub energy: f32,

    pub stress: f32,

    pub recovery: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicAction {
    pub subsystem: String,

    pub cost: f32,
}

pub struct SwarmMetabolism;

impl SwarmMetabolism {
    pub fn process(state: &mut MetabolicState, actions: &[MetabolicAction]) {
        for action in actions {
            println!("[METABOLISM] {} consumed {}", action.subsystem, action.cost);

            state.energy -= action.cost;

            state.stress += action.cost * 0.4;
        }

        if state.energy < 0.30 {
            println!("[METABOLISM] low energy state");

            state.recovery += 0.5;
        }

        if state.recovery > 0.80 {
            println!("[METABOLISM] initiating recovery cycle");

            state.energy += 0.4;

            state.stress *= 0.5;
        }
    }
}
