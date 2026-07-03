//! Pandora Self Healing — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeHealth {
    pub subsystem: String,

    pub stability: f64,

    pub repair_success: f64,

    pub survivability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingDirective {
    pub subsystem: String,

    pub action: String,

    pub urgency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingPlan {
    pub stable: bool,

    pub directives: Vec<HealingDirective>,
}

pub struct RuntimeSelfHealingCoordinator;

impl RuntimeSelfHealingCoordinator {
    pub fn stabilize(runtime: &[RuntimeHealth]) -> HealingPlan {
        let mut directives = Vec::new();

        let mut stable = true;

        for subsystem in runtime {
            println!(
                "[HEALING] subsystem={} stability={}",
                subsystem.subsystem, subsystem.stability
            );

            if subsystem.stability < 0.75 {
                stable = false;

                directives.push(HealingDirective {
                    subsystem: subsystem.subsystem.clone(),

                    action: "trigger recursive repair loop".into(),

                    urgency: 0.92,
                });
            }

            if subsystem.repair_success < 0.70 {
                stable = false;

                directives.push(HealingDirective {
                    subsystem: subsystem.subsystem.clone(),

                    action: "increase repair redundancy".into(),

                    urgency: 0.81,
                });
            }

            if subsystem.survivability < 0.72 {
                stable = false;

                directives.push(HealingDirective {
                    subsystem: subsystem.subsystem.clone(),

                    action: "migrate workload topology".into(),

                    urgency: 0.87,
                });
            }
        }

        HealingPlan { stable, directives }
    }
}
