use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeCheckpoint {
    pub checkpoint_id: String,

    pub runtime_state: String,

    pub active_nodes: usize,

    pub execution_graph_nodes: usize,
}

pub struct CheckpointCoordinator;

impl CheckpointCoordinator {
    pub fn persist(checkpoint: &RuntimeCheckpoint) {
        fs::create_dir_all("checkpoints").unwrap();

        let path = format!("checkpoints/{}.json", checkpoint.checkpoint_id);

        let content = serde_json::to_string_pretty(checkpoint).unwrap();

        fs::write(path, content).unwrap();

        println!("[CHECKPOINT] persisted {}", checkpoint.checkpoint_id);
    }

    pub fn recover(checkpoint_id: &str) -> Option<RuntimeCheckpoint> {
        let path = format!("checkpoints/{}.json", checkpoint_id);

        let content = fs::read_to_string(path).ok()?;

        serde_json::from_str(&content).ok()
    }
}
