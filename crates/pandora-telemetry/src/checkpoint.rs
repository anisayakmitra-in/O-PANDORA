//! Checkpoint — absorbed from pandora-checkpoint (Phase 1C).
//!
//! Pandora Checkpoint — runtime checkpoint persistence and recovery.
//!
//! Phase 1A decomposition: extracted from pandora-runtime/src/checkpoint.rs.

use serde::{Deserialize, Serialize};
use std::fs;

/// A snapshot of runtime state at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeCheckpoint {
    pub checkpoint_id: String,
    pub runtime_state: String,
    pub active_nodes: usize,
    pub execution_graph_nodes: usize,
}

/// Coordinates persistence and recovery of runtime checkpoints.
pub struct CheckpointCoordinator;

impl CheckpointCoordinator {
    /// Persist a checkpoint to disk under a `checkpoints/` directory.
    pub fn persist(checkpoint: &RuntimeCheckpoint) {
        let _ = fs::create_dir_all("checkpoints");
        let path = format!("checkpoints/{}.json", checkpoint.checkpoint_id);
        if let Ok(content) = serde_json::to_string_pretty(checkpoint) {
            let _ = fs::write(path, content);
        }
    }

    /// Recover a checkpoint from disk by ID.
    pub fn recover(checkpoint_id: &str) -> Option<RuntimeCheckpoint> {
        let path = format!("checkpoints/{}.json", checkpoint_id);
        let content = fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }
}
