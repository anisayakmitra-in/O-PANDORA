//! Pandora Rollback — extracted from pandora-runtime (Phase 1B).
//!
use crate::checkpoint::RuntimeCheckpoint;

pub struct RollbackEngine;

impl RollbackEngine {
    pub fn recover(checkpoints: &[RuntimeCheckpoint]) -> Option<RuntimeCheckpoint> {
        checkpoints.last().cloned()
    }
}
