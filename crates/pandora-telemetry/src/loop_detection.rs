//! Loop Detection — detects repetitive patterns.
//!
//! ponytail: simplified after entropy module removal. Uses String instead of ToolCall.

pub struct LoopDetector;

impl LoopDetector {
    pub fn detect_repetition(calls: &[String], threshold: usize) -> bool {
        if calls.len() < threshold {
            return false;
        }
        let recent = &calls[calls.len() - threshold..];
        let first = &recent[0];
        recent.iter().all(|c| c == first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_repetition() {
        let calls = vec!["ls".into(), "ls".into(), "ls".into()];
        assert!(LoopDetector::detect_repetition(&calls, 3));
    }

    #[test]
    fn no_false_positive() {
        let calls = vec!["ls".into(), "pwd".into(), "ls".into()];
        assert!(!LoopDetector::detect_repetition(&calls, 3));
    }
}
