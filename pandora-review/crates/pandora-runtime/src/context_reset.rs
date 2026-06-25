use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextState {
    pub active_context: usize,

    pub entropy: f32,

    pub reset_triggered: bool,
}

pub struct ContextResetEngine;

impl ContextResetEngine {
    pub fn evaluate(state: &mut ContextState) {
        if state.entropy > 1.5 || state.active_context > 10 {
            state.reset_triggered = true;
        }
    }
}
