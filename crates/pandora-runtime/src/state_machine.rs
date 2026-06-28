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

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum ExecutionStateKind {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl ExecutionStateMachine {
    pub fn transition_kind(
        task_id: &str,
        previous: ExecutionStateKind,
        current: ExecutionStateKind,
    ) -> StateTransitionKind {
        println!("[STATE] {} {:?} -> {:?}", task_id, previous, current);
        StateTransitionKind {
            task_id: task_id.into(),
            previous,
            current,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateTransitionKind {
    pub task_id: String,
    pub previous: ExecutionStateKind,
    pub current: ExecutionStateKind,
}
