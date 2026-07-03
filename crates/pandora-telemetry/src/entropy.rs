//! Entropy calculation and ToolCall primitives.
//!
//! Absorbed from pandora-telemetry micro-crate (Phase 1C consolidation).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A record of a tool invocation during execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool: String,
}

impl ToolCall {
    pub fn new(tool: impl Into<String>) -> Self {
        Self { tool: tool.into() }
    }
}

/// Computes entropy over a sequence of tool calls.
pub struct EntropyEngine;

impl EntropyEngine {
    pub fn calculate_entropy(calls: &[ToolCall]) -> f64 {
        if calls.is_empty() {
            return 0.0;
        }
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for call in calls {
            *counts.entry(&call.tool).or_insert(0) += 1;
        }
        let total = calls.len() as f64;
        let mut entropy = 0.0;
        for count in counts.values() {
            let p = *count as f64 / total;
            entropy -= p * p.log2();
        }
        entropy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_call_creation() {
        let call = ToolCall::new("grep");
        assert_eq!(call.tool, "grep");
    }

    #[test]
    fn entropy_zero_for_same_tool() {
        let calls = vec![
            ToolCall::new("grep"),
            ToolCall::new("grep"),
            ToolCall::new("grep"),
        ];
        let entropy = EntropyEngine::calculate_entropy(&calls);
        assert!((entropy - 0.0).abs() < 1e-10);
    }

    #[test]
    fn entropy_max_for_even_distribution() {
        let calls = vec![
            ToolCall::new("a"),
            ToolCall::new("b"),
        ];
        let entropy = EntropyEngine::calculate_entropy(&calls);
        assert!((entropy - 1.0).abs() < 1e-10);
    }

    #[test]
    fn entropy_empty_returns_zero() {
        let calls: Vec<ToolCall> = vec![];
        let entropy = EntropyEngine::calculate_entropy(&calls);
        assert!((entropy - 0.0).abs() < 1e-10);
    }
}
