use crate::checkpoint::CognitionCheckpoint;

pub struct RollbackEngine;

impl RollbackEngine {
    pub fn recover(checkpoints: &[CognitionCheckpoint]) -> Option<CognitionCheckpoint> {
        checkpoints
            .iter()
            .rev()
            .find(|checkpoint| checkpoint.stable)
            .cloned()
    }
}
