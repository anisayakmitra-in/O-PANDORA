use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveTask {
    pub task_id: String,

    pub objective: String,

    pub depth: usize,
}

pub struct RecursivePlanner;

impl RecursivePlanner {
    pub fn expand(task: &RecursiveTask) -> Vec<RecursiveTask> {
        println!(
            "[RECURSIVE] expanding {} depth={}",
            task.task_id, task.depth
        );

        if task.depth >= 3 {
            return vec![];
        }

        vec![
            RecursiveTask {
                task_id: format!("{}-a", task.task_id),

                objective: format!("analyze {}", task.objective),

                depth: task.depth + 1,
            },
            RecursiveTask {
                task_id: format!("{}-b", task.task_id),

                objective: format!("execute {}", task.objective),

                depth: task.depth + 1,
            },
        ]
    }

    pub fn recurse(task: RecursiveTask) {
        println!(
            "[RECURSIVE] task={} objective={}",
            task.task_id, task.objective
        );

        let children = Self::expand(&task);

        for child in children {
            Self::recurse(child);
        }
    }
}
