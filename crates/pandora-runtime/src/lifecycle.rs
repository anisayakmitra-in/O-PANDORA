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
