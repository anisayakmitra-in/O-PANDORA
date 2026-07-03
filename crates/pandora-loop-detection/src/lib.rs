//! Pandora Loop Detection — detects repetitive tool call patterns.
//!
//! Phase 1A decomposition: extracted from pandora-runtime/src/loop_detection.rs.

use pandora_telemetry::ToolCall;

/// Detects repetitive tool call sequences.
pub struct LoopDetector;

impl LoopDetector {
    /// Returns true if the last `threshold` calls all use the same tool.
    pub fn detect_repetition(calls: &[ToolCall], threshold: usize) -> bool {
        if calls.len() < threshold {
            return false;
        }
        let recent = &calls[calls.len() - threshold..];
        let first = &recent[0].tool;
        recent.iter().all(|call| &call.tool == first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_repetition() {
        let calls = vec![
            ToolCall::new("grep"),
            ToolCall::new("grep"),
            ToolCall::new("grep"),
        ];
        assert!(LoopDetector::detect_repetition(&calls, 3));
    }

    #[test]
    fn no_repetition_below_threshold() {
        let calls = vec![ToolCall::new("grep")];
        assert!(!LoopDetector::detect_repetition(&calls, 3));
    }

    #[test]
    fn different_tools_not_repetitive() {
        let calls = vec![
            ToolCall::new("grep"),
            ToolCall::new("find"),
            ToolCall::new("grep"),
        ];
        assert!(!LoopDetector::detect_repetition(&calls, 3));
    }
}
