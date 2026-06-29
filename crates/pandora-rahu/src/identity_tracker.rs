//! Identity Tracker (HADES integration).
//!
//! Updates identity continuity, lineage, soul state,
//! and personality drift after every execution.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use pandora_types::identity_runtime::{IdentityUpdate, ResurrectionState};

/// Tracks identity state updates for HADES.
pub struct IdentityTracker {
    updates: Arc<Mutex<Vec<IdentityUpdate>>>,
    current_state: Arc<Mutex<BTreeMap<String, ResurrectionState>>>,
}

impl IdentityTracker {
    pub fn new() -> Self {
        IdentityTracker {
            updates: Arc::new(Mutex::new(vec![])),
            current_state: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    /// Record an identity update after execution.
    pub fn record(&self, update: IdentityUpdate) {
        self.current_state
            .lock()
            .unwrap()
            .insert(update.identity_id.clone(), update.resurrection_state);
        self.updates.lock().unwrap().push(update);
    }

    /// Get current resurrection state for an identity.
    pub fn state(&self, identity_id: &str) -> Option<ResurrectionState> {
        self.current_state.lock().unwrap().get(identity_id).copied()
    }

    /// Get all recorded updates.
    pub fn history(&self) -> Vec<IdentityUpdate> {
        self.updates.lock().unwrap().clone()
    }
}

impl Default for IdentityTracker {
    fn default() -> Self {
        Self::new()
    }
}
