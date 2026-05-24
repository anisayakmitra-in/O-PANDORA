use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedTask {
    pub task_id: String,

    pub objective: String,

    pub priority: u32,
}

pub struct AutonomousTaskSpawner;

impl AutonomousTaskSpawner {
    pub fn spawn(objective: &str, count: usize) -> Vec<SpawnedTask> {
        let mut tasks = Vec::new();

        for i in 0..count {
            let task = SpawnedTask {
                task_id: format!("spawned_{}", i),

                objective: format!("{} :: branch {}", objective, i),

                priority: (count - i) as u32,
            };

            println!("[SPAWNER] spawned {}", task.task_id);

            tasks.push(task);
        }

        tasks
    }
}
