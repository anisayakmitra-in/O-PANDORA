//! Pandora Context Router — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMemory {
    pub memory_id: String,

    pub relevance: f64,

    pub token_cost: usize,

    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutedContext {
    pub selected: Vec<ContextMemory>,

    pub total_tokens: usize,
}

pub struct ContextRoutingEngine;

impl ContextRoutingEngine {
    pub fn route(memories: &[ContextMemory], max_tokens: usize) -> RoutedContext {
        let mut sorted = memories.to_vec();

        sorted.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());

        let mut selected = Vec::new();

        let mut tokens = 0;

        for memory in sorted {
            if tokens + memory.token_cost > max_tokens {
                continue;
            }

            println!("[CONTEXT] routing {}", memory.memory_id);

            tokens += memory.token_cost;

            selected.push(memory);
        }

        RoutedContext {
            selected,

            total_tokens: tokens,
        }
    }
}
