use serde::{Deserialize, Serialize};

use crate::coding_engine::{AutonomousCodingEngine, CodePatch};

use crate::semantic_patch::SemanticPatch;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairExecutionResult {
    pub successful: usize,

    pub failed: usize,
}

pub struct RepairExecutionCoordinator;

impl RepairExecutionCoordinator {
    pub fn execute(patches: &[SemanticPatch]) -> RepairExecutionResult {
        let mut successful = 0;

        let mut failed = 0;

        for patch in patches {
            println!("[REPAIR-EXEC] applying patch {}", patch.target_file);

            let code_patch = CodePatch {
                target_file: patch.target_file.clone(),

                search: patch.search.clone(),

                replace: patch.replace.clone(),
            };

            let result = AutonomousCodingEngine::apply_patch(&code_patch);

            if result.success {
                successful += 1;

                println!("[REPAIR-EXEC] patch successful");
            } else {
                failed += 1;

                println!("[REPAIR-EXEC] patch failed {}", result.output);
            }
        }

        RepairExecutionResult { successful, failed }
    }
}
