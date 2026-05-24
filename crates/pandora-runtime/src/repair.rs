use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairAction {
    pub subsystem: String,

    pub action: String,

    pub severity: String,
}

pub struct AutonomousRepairCoordinator;

impl AutonomousRepairCoordinator {
    pub fn evaluate(subsystem: &str, degraded: bool) -> Option<RepairAction> {
        if degraded {
            println!("[REPAIR] instability detected in {}", subsystem);

            return Some(RepairAction {
                subsystem: subsystem.into(),

                action: "rollback_and_restart".into(),

                severity: "high".into(),
            });
        }

        None
    }

    pub fn execute(repair: &RepairAction) {
        println!(
            "[REPAIR] executing {} on {}",
            repair.action, repair.subsystem
        );
    }
}
