use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub step_id: String,

    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub goal: String,

    pub steps: Vec<ExecutionStep>,
}

pub struct Planner;

impl Planner {
    pub fn generate(goal: &str) -> ExecutionPlan {
        let mut steps = Vec::new();

        steps.push(ExecutionStep {
            step_id: "step_001".into(),

            description: "analyze objective".into(),
        });

        steps.push(ExecutionStep {
            step_id: "step_002".into(),

            description: "allocate execution agents".into(),
        });

        steps.push(ExecutionStep {
            step_id: "step_003".into(),

            description: "execute workflow".into(),
        });

        steps.push(ExecutionStep {
            step_id: "step_004".into(),

            description: "evaluate outcomes".into(),
        });

        ExecutionPlan {
            goal: goal.into(),

            steps,
        }
    }
}
