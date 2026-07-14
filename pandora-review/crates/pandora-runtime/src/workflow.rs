use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub step_id: String,

    pub action: String,

    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurableWorkflow {
    pub workflow_id: String,

    pub steps: Vec<WorkflowStep>,
}

pub struct WorkflowEngine;

impl WorkflowEngine {
    pub fn persist(workflow: &DurableWorkflow) {
        fs::create_dir_all("workflows").unwrap();

        let path = format!("workflows/{}.json", workflow.workflow_id);

        let content = serde_json::to_string_pretty(workflow).unwrap();

        fs::write(path, content).unwrap();

        println!("[WORKFLOW] persisted {}", workflow.workflow_id);
    }

    pub fn execute(workflow: &mut DurableWorkflow) {
        println!("[WORKFLOW] executing {}", workflow.workflow_id);

        for step in workflow.steps.iter_mut() {
            println!("[WORKFLOW] step {} -> {}", step.step_id, step.action);

            step.completed = true;
        }
    }
}
