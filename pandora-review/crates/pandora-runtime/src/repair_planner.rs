use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureContext {
    pub subsystem: String,

    pub error: String,

    pub severity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairPlan {
    pub strategy: String,

    pub priority: f32,

    pub actions: Vec<String>,
}

pub struct AutonomousRepairPlanner;

impl AutonomousRepairPlanner {
    pub fn plan(context: &FailureContext) -> RepairPlan {
        println!("[REPAIR] analyzing {}", context.subsystem);

        let mut actions = Vec::new();

        if context.error.contains("unresolved import") {
            actions.push("analyze dependency graph".into());

            actions.push("repair module exports".into());
        }

        if context.error.contains("cannot find type") {
            actions.push("scan AST definitions".into());

            actions.push("repair type references".into());
        }

        if context.severity > 0.80 {
            actions.push("trigger rollback checkpoint".into());
        }

        let strategy = if context.severity > 0.70 {
            "stabilization-first"
        } else {
            "incremental-repair"
        };

        RepairPlan {
            strategy: strategy.into(),

            priority: context.severity,

            actions,
        }
    }
}
