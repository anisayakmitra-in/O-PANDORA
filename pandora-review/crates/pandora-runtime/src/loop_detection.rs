use crate::telemetry::ToolCall;

pub struct LoopDetector;

impl LoopDetector {
    pub fn detect_repetition(calls: &[ToolCall], threshold: usize) -> bool {
        if calls.len() < threshold {
            return false;
        }

        let recent = &calls[calls.len() - threshold..];

        let first = &recent[0].tool;

        recent.iter().all(|call| &call.tool == first)
    }
}
