//! Pandora Replay — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaySession {
    pub replay_id: String,

    pub target_graph: String,

    pub target_lineage: String,

    pub target_mutation: String,

    pub replay_reason: String,

    pub initiated_by: String,

    pub timestamp: String,
}
