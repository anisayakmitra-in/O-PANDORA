use serde::{Deserialize, Serialize};

use crate::swarm_metabolism::MetabolicState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeostasisAdjustment {
    pub action: String,

    pub intensity: f32,
}

pub struct SwarmHomeostasis;

impl SwarmHomeostasis {
    pub fn stabilize(state: &MetabolicState) -> Vec<HomeostasisAdjustment> {
        let mut adjustments = Vec::new();

        if state.stress > 0.75 {
            adjustments.push(HomeostasisAdjustment {
                action: "reduce_swarm_activity".into(),

                intensity: 0.6,
            });
        }

        if state.energy < 0.40 {
            adjustments.push(HomeostasisAdjustment {
                action: "initiate_recovery_cycle".into(),

                intensity: 0.8,
            });
        }

        if state.recovery > 0.85 {
            adjustments.push(HomeostasisAdjustment {
                action: "resume_normal_execution".into(),

                intensity: 0.4,
            });
        }

        println!("[HOMEOSTASIS] adjustments={}", adjustments.len());

        adjustments
    }
}
