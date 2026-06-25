use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionState {
    Pending,

    Scheduled,

    Running,

    Completed,

    Failed,

    Recovered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub task_id: String,

    pub previous: ExecutionState,

    pub current: ExecutionState,
}

pub struct ExecutionStateMachine;

impl ExecutionStateMachine {
    pub fn transition(
        task_id: &str,

        previous: ExecutionState,

        current: ExecutionState,
    ) -> StateTransition {
        println!("[STATE] {} {:?} -> {:?}", task_id, previous, current);

        StateTransition {
            task_id: task_id.into(),

            previous,

            current,
        }
    }
}
