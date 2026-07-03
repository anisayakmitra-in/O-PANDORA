//! Memory Prompting — consolidated into pandora-discovery.
//!
use serde::{Deserialize, Serialize};

use crate::context_router::RoutedContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRequest {
    pub system_goal: String,

    pub workload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedPrompt {
    pub prompt: String,

    pub injected_memories: usize,

    pub estimated_tokens: usize,
}

pub struct MemoryAwarePromptEngine;

impl MemoryAwarePromptEngine {
    pub fn construct(request: &PromptRequest, routed: &RoutedContext) -> ConstructedPrompt {
        println!("[PROMPT] constructing prompt");

        let mut prompt = String::new();

        prompt.push_str("SYSTEM GOAL:\n");

        prompt.push_str(&request.system_goal);

        prompt.push_str("\n\nWORKLOAD:\n");

        prompt.push_str(&request.workload);

        prompt.push_str("\n\nMEMORY CONTEXT:\n");

        for memory in &routed.selected {
            prompt.push_str("\n---\n");

            prompt.push_str(&memory.content);
        }

        let estimated_tokens = prompt.len() / 4;

        ConstructedPrompt {
            prompt,

            injected_memories: routed.selected.len(),

            estimated_tokens,
        }
    }
}
