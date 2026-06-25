use serde::{Deserialize, Serialize};

use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurableTask {
    pub task_id: String,

    pub task_type: String,

    pub payload: String,

    pub retry_count: u32,
}

pub struct DurableQueue;

impl DurableQueue {
    pub fn persist(task: &DurableTask) {
        fs::create_dir_all("queue").unwrap();

        let path = format!("queue/{}.json", task.task_id);

        let content = serde_json::to_string_pretty(task).unwrap();

        fs::write(path, content).unwrap();

        println!("[QUEUE] persisted task: {}", task.task_id);
    }

    pub fn recover() -> Vec<DurableTask> {
        let mut tasks = Vec::new();

        if !Path::new("queue").exists() {
            return tasks;
        }

        for entry in fs::read_dir("queue").unwrap() {
            let entry = entry.unwrap();

            let content = fs::read_to_string(entry.path()).unwrap();

            let task: DurableTask = serde_json::from_str(&content).unwrap();

            tasks.push(task);
        }

        tasks
    }
}
