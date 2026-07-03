//! Absorbed from pandora-debugging-loop (Phase 1C).
//!
//! Pandora Debugging Loop — extracted from pandora-runtime (Phase 1B).
//!
use serde::{Deserialize, Serialize};

use pandora_runtime::repair_planner::{AutonomousRepairPlanner, FailureContext};

use pandora_runtime::semantic_patch::{SemanticIssue, SemanticPatchPlanner};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugCycle {
    pub cycle: usize,

    pub issue: String,

    pub repaired: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebuggingResult {
    pub resolved: bool,

    pub cycles: usize,

    pub history: Vec<DebugCycle>,
}

pub struct AutonomousDebugLoop;

impl AutonomousDebugLoop {
    pub fn execute(issue: &SemanticIssue, max_cycles: usize) -> DebuggingResult {
        let mut history = Vec::new();

        let mut resolved = false;

        for cycle in 0..max_cycles {
            println!("[DEBUG] cycle={}", cycle + 1);

            let failure = FailureContext {
                subsystem: "runtime".into(),

                error: issue.issue.clone(),

                severity: issue.severity,
            };

            let repair_plan = AutonomousRepairPlanner::plan(&failure);

            println!("[DEBUG] strategy={}", repair_plan.strategy);

            let patches = SemanticPatchPlanner::generate(issue);

            let repaired = !patches.is_empty();

            history.push(DebugCycle {
                cycle: cycle + 1,

                issue: issue.issue.clone(),

                repaired,
            });

            if repaired {
                resolved = true;

                println!("[DEBUG] issue resolved");

                break;
            }
        }

        DebuggingResult {
            resolved,

            cycles: history.len(),

            history,
        }
    }
}
