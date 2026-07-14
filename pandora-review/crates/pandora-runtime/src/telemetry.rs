use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool: String,
}

pub struct EntropyEngine;

impl EntropyEngine {
    pub fn calculate_entropy(calls: &[ToolCall]) -> f32 {
        if calls.is_empty() {
            return 0.0;
        }

        let mut counts: HashMap<String, usize> = HashMap::new();

        for call in calls {
            *counts.entry(call.tool.clone()).or_insert(0) += 1;
        }

        let total = calls.len() as f32;

        let mut entropy = 0.0;

        for count in counts.values() {
            let p = *count as f32 / total;

            entropy -= p * p.log2();
        }

        entropy
    }
}
