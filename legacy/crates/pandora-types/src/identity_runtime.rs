//! Identity runtime types — execution updates identity continuity,
//! lineage, soul state, personality drift, and fork detection.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Identity state update after an execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityUpdate {
    pub session_id: String,
    pub identity_id: String,
    pub continuity_score: f64,
    pub personality_drift: f64,
    pub fork_detected: bool,
    pub lineage_depth: u32,
    pub resurrection_state: ResurrectionState,
    pub metadata: BTreeMap<String, String>,
    pub timestamp_ms: u64,
}

/// Resurrection state for identity recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResurrectionState {
    Alive,
    Dormant,
    Suspended,
    Recovering,
    Lost,
}
