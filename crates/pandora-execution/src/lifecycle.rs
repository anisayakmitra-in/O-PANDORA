//! Absorbed from pandora-lifecycle (Phase 1C).
//!
//! Pandora Lifecycle — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeState {
    Initializing,

    Running,

    Recovering,

    Degraded,

    Shutdown,
}

pub struct LifecycleManager {
    pub state: RuntimeState,
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LifecycleManager {
    pub fn new() -> Self {
        Self {
            state: RuntimeState::Initializing,
        }
    }

    pub fn transition(&mut self, state: RuntimeState) {
        self.state = state.clone();

        println!("[LIFECYCLE] transitioned to {:?}", state);
    }

    pub fn current(&self) -> RuntimeState {
        self.state.clone()
    }
}
