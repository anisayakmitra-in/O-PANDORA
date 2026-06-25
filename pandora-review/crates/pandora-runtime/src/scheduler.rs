use serde::{Deserialize, Serialize};

use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskState {
    Pending,

    Running,

    Sleeping,

    Failed,

    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionTask {
    pub task_id: String,

    pub task_type: String,

    pub retries: u32,

    pub max_retries: u32,

    pub budget_ms: u64,

    pub recurring: bool,

    pub wake_at: Option<u64>,

    pub state: TaskState,
}

pub struct CognitionScheduler {
    pub queue: VecDeque<CognitionTask>,
}

impl CognitionScheduler {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, task: CognitionTask) {
        self.queue.push_back(task);
    }

    pub fn heartbeat(&mut self) {
        println!("[SCHEDULER] heartbeat pulse");

        for task in self.queue.iter_mut() {
            match task.state {
                TaskState::Pending => {
                    println!("[SCHEDULER] running task: {}", task.task_id);

                    task.state = TaskState::Running;
                }

                TaskState::Running => {
                    println!("[SCHEDULER] completed task: {}", task.task_id);

                    if task.recurring {
                        task.state = TaskState::Pending;
                    } else {
                        task.state = TaskState::Completed;
                    }
                }

                TaskState::Failed => {
                    if task.retries < task.max_retries {
                        task.retries += 1;

                        println!("[SCHEDULER] retrying task: {}", task.task_id);

                        task.state = TaskState::Pending;
                    } else {
                        println!("[WATCHDOG] task exceeded retry budget: {}", task.task_id);
                    }
                }

                TaskState::Sleeping => {
                    println!("[SCHEDULER] sleeping task: {}", task.task_id);
                }

                TaskState::Completed => {}
            }

            if task.budget_ms > 10_000 {
                println!("[WATCHDOG] excessive budget detected: {}", task.task_id);
            }
        }
    }
}
