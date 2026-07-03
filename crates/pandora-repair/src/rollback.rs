//! Absorbed from pandora-rollback (Phase 1C).
//!
//! Pandora Rollback — extracted from pandora-runtime (Phase 1B).
//!
use pandora_runtime::checkpoint::RuntimeCheckpoint;

pub struct RollbackEngine;

impl RollbackEngine {
    pub fn recover(checkpoints: &[RuntimeCheckpoint]) -> Option<RuntimeCheckpoint> {
        checkpoints.last().cloned()
    }
}
