//! Pandora Swarm Will — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WillDirective {
    pub objective: String,

    pub persistence: f32,

    pub priority: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WillState {
    pub dominant_objective: String,

    pub determination: f32,

    pub strategic_pressure: f32,
}

pub struct SwarmWill;

impl SwarmWill {
    pub fn synthesize(directives: &[WillDirective]) -> WillState {
        let mut dominant = "survival".to_string();

        let mut determination = 0.0;

        let mut pressure = 0.0;

        for directive in directives {
            println!(
                "[WILL] objective={} persistence={} priority={}",
                directive.objective, directive.persistence, directive.priority
            );

            let score = directive.persistence * directive.priority;

            pressure += score;

            if score > determination {
                determination = score;

                dominant = directive.objective.clone();
            }
        }

        WillState {
            dominant_objective: dominant,

            determination,

            strategic_pressure: pressure,
        }
    }
}
