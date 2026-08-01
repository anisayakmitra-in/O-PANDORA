//! Checkpoint Manager — crash recovery for execution pipeline.
//!
//! After each pipeline stage, a checkpoint is written to disk. If Pandora
//! crashes mid-execution, the next `pandora resume` picks up from the last
//! completed stage rather than starting over.
//!
//! Design: append-only log of checkpoints, one per stage per execution.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// A single checkpoint — which stage completed, with what state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub execution_id: String,
    pub stage: PipelineStage,
    pub timestamp: u64,
    pub data_hash: Option<String>, // hash of stage output
}

#[non_exhaustive]
/// Ordered pipeline stages (must match orchestrator flow).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineStage {
    Plan = 1,
    Workflow = 2,
    Council = 3,
    Policy = 4,
    Resolution = 5,
    Execution = 6,
    Recorder = 7,
    Telemetry = 8,
    Intel = 9,
    Ledger = 10,
    Complete = 11,
}

impl PipelineStage {
    pub fn number(&self) -> u8 {
        *self as u8
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::Plan => "plan",
            Self::Workflow => "workflow",
            Self::Council => "council",
            Self::Policy => "policy",
            Self::Resolution => "resolution",
            Self::Execution => "execution",
            Self::Recorder => "recorder",
            Self::Telemetry => "telemetry",
            Self::Intel => "intel",
            Self::Ledger => "ledger",
            Self::Complete => "complete",
        }
    }
}

/// Manages crash recovery checkpoints.
#[derive(Debug, Default)]
pub struct CheckpointManager {
    dir: PathBuf,
}

impl CheckpointManager {
    pub fn new() -> Self {
        let dir = std::env::var_os("PANDORA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("USERPROFILE").map(PathBuf::from))
            .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
            .map(|root| {
                if std::env::var_os("PANDORA_HOME").is_some() {
                    root.join("checkpoints")
                } else {
                    root.join(".pandora/checkpoints")
                }
            })
            .unwrap_or_else(|| PathBuf::from(".pandora/checkpoints"));
        std::fs::create_dir_all(&dir).ok();
        Self { dir }
    }

    /// Save a checkpoint after a stage completes.
    pub fn save(&self, execution_id: &str, stage: PipelineStage, data_hash: Option<&str>) {
        let cp = Checkpoint {
            execution_id: execution_id.into(),
            stage,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            data_hash: data_hash.map(String::from),
        };
        let path = self.path(execution_id);
        let mut checkpoints: Vec<Checkpoint> = self.load_raw(execution_id);
        checkpoints.push(cp);
        if let Ok(json) = serde_json::to_string(&checkpoints) {
            let _ = std::fs::write(&path, json);
        }
    }

    /// Get the last completed stage for an execution.
    pub fn last_stage(&self, execution_id: &str) -> Option<PipelineStage> {
        let checkpoints = self.load_raw(execution_id);
        checkpoints.last().map(|cp| cp.stage)
    }

    /// Check if an execution is still in-progress (not Complete).
    pub fn is_in_progress(&self, execution_id: &str) -> bool {
        self.last_stage(execution_id)
            .map(|s| s != PipelineStage::Complete)
            .unwrap_or(false)
    }

    /// List all in-progress (incomplete) executions.
    pub fn in_progress(&self) -> Vec<String> {
        let mut ids = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    let id = name.trim_end_matches(".json");
                    if self.is_in_progress(id) {
                        ids.push(id.to_string());
                    }
                }
            }
        }
        ids
    }

    /// Clean up checkpoints for a completed execution.
    pub fn complete(&self, execution_id: &str) {
        self.save(execution_id, PipelineStage::Complete, None);
    }

    /// Remove all checkpoints for an execution.
    pub fn clear(&self, execution_id: &str) {
        let _ = std::fs::remove_file(self.path(execution_id));
    }

    fn path(&self, execution_id: &str) -> PathBuf {
        self.dir.join(format!("{execution_id}.json"))
    }

    fn load_raw(&self, execution_id: &str) -> Vec<Checkpoint> {
        std::fs::read_to_string(self.path(execution_id))
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_save_and_load() {
        let _guard = crate::test_support::process_env_lock();
        let root = std::env::temp_dir().join("pandora-checkpoint-save");
        let _ = std::fs::remove_dir_all(&root);
        std::env::set_var("PANDORA_HOME", &root);
        let cm = CheckpointManager::new();
        cm.save("test-exec", PipelineStage::Plan, None);
        assert_eq!(cm.last_stage("test-exec"), Some(PipelineStage::Plan));
        cm.clear("test-exec");
        let _ = std::fs::remove_dir_all(&root);
        std::env::remove_var("PANDORA_HOME");
    }

    #[test]
    fn incomplete_detection() {
        let _guard = crate::test_support::process_env_lock();
        let root = std::env::temp_dir().join("pandora-checkpoint-incomplete");
        let _ = std::fs::remove_dir_all(&root);
        std::env::set_var("PANDORA_HOME", &root);
        let cm = CheckpointManager::new();
        cm.save("test-incomplete", PipelineStage::Execution, None);
        assert!(cm.is_in_progress("test-incomplete"));
        cm.clear("test-incomplete");
        let _ = std::fs::remove_dir_all(&root);
        std::env::remove_var("PANDORA_HOME");
    }

    #[test]
    fn complete_execution() {
        let cm = CheckpointManager::new();
        cm.complete("test-done");
        assert!(!cm.is_in_progress("test-done"));
        cm.clear("test-done");
    }

    #[test]
    fn stage_ordering() {
        assert!(PipelineStage::Plan.number() < PipelineStage::Complete.number());
        assert_eq!(PipelineStage::Complete.number(), 11);
    }
}
