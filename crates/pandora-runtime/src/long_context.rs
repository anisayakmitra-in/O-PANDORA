use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindow {
    pub window_id: String,

    pub token_usage: usize,

    pub priority: f64,

    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratedContext {
    pub active_windows: Vec<ContextWindow>,

    pub archived_windows: Vec<ContextWindow>,

    pub total_tokens: usize,
}

pub struct LongContextOrchestrator;

impl LongContextOrchestrator {
    pub fn orchestrate(windows: &[ContextWindow], max_tokens: usize) -> OrchestratedContext {
        let mut sorted = windows.to_vec();

        sorted.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());

        let mut active = Vec::new();

        let mut archived = Vec::new();

        let mut total = 0;

        for window in sorted {
            if total + window.token_usage <= max_tokens {
                println!("[LONGCTX] activating {}", window.window_id);

                total += window.token_usage;

                active.push(window);
            } else {
                println!("[LONGCTX] archiving {}", window.window_id);

                archived.push(window);
            }
        }

        OrchestratedContext {
            active_windows: active,

            archived_windows: archived,

            total_tokens: total,
        }
    }
}
