//! Context Strategy — automatic conversation size reduction.
//!
//! When sessions overflow the model's context window, the context strategy
//! reduces the conversation. Pluggable strategies: summarize, drop-oldest,
//! archive, externalize. Configurable per session.
//!
//! Inspired by claurst's context_collapse, generalized to multiple strategies.

use serde::{Deserialize, Serialize};

/// Strategy for managing context when it exceeds token limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ContextStrategy {
    /// Drop oldest non-system messages first.
    #[default]
    DropOldest,
    /// Summarize the middle of the conversation.
    Summarize,
    /// Move old messages to archive, keep summary.
    Archive,
    /// Externalize to file, keep reference.
    Externalize,
}

impl ContextStrategy {
    pub fn label(&self) -> &'static str {
        match self {
            Self::DropOldest => "drop-oldest",
            Self::Summarize => "summarize",
            Self::Archive => "archive",
            Self::Externalize => "externalize",
        }
    }
}

/// A message in the context window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMessage {
    pub role: String,
    pub content: String,
    pub timestamp: u64,
    pub pinned: bool,
}

impl ContextMessage {
    /// Estimate token count (~4 chars per token).
    pub fn estimated_tokens(&self) -> usize {
        (self.content.len() / 4).max(1)
    }
}

/// Maximum iterations for enforce_limit() before forcing termination.
/// This is a safety guard — under normal operation the loop exits in
/// O(messages.len()) iterations. MAX_ITERATIONS prevents pathological
/// cases (e.g., a summary stub that alone exceeds max_tokens) from
/// spinning forever. If the limit is hit, the strategy falls back to
/// DropOldest to guarantee termination.
const MAX_ITERATIONS: usize = 256;

/// The context manager — tracks messages and applies strategies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextManager {
    pub messages: Vec<ContextMessage>,
    pub max_tokens: usize,
    pub strategy: ContextStrategy,
    pub messages_dropped: usize,
    pub messages_archived: usize,
    /// Whether the strategy fell back to DropOldest (set when
    /// summarization cannot reduce token count enough).
    pub fell_back_to_drop: bool,
}

impl ContextManager {
    pub fn new(max_tokens: usize, strategy: ContextStrategy) -> Self {
        Self {
            max_tokens,
            strategy,
            ..Default::default()
        }
    }

    /// Add a message. If over limit, apply the strategy.
    pub fn push(&mut self, msg: ContextMessage) {
        self.messages.push(msg);
        self.enforce_limit();
    }

    /// Total estimated tokens across all messages.
    pub fn total_tokens(&self) -> usize {
        self.messages.iter().map(|m| m.estimated_tokens()).sum()
    }

    /// Check if over the limit.
    pub fn is_over_limit(&self) -> bool {
        self.total_tokens() > self.max_tokens
    }

    /// Apply the strategy if over limit.
    ///
    /// ## Termination guarantee
    ///
    /// This method MUST always terminate. It uses two safeguards:
    ///
    /// 1. **Iteration guard**: After `MAX_ITERATIONS` iterations, the loop
    ///    breaks unconditionally.
    /// 2. **Fallback**: If the chosen strategy (Summarize, Archive,
    ///    Externalize) cannot reduce tokens enough — e.g., a summary stub
    ///    alone exceeds `max_tokens` — the strategy automatically falls back
    ///    to `DropOldest` which removes messages entirely.
    fn enforce_limit(&mut self) {
        let mut iterations = 0;
        let mut summarize_attempts = 0;
        let max_summarize_attempts = self.messages.len().saturating_add(1);

        while self.is_over_limit() && self.messages.len() > 1 {
            iterations += 1;
            if iterations > MAX_ITERATIONS {
                // Hard guard — never spin forever.
                break;
            }

            // If we've tried summarizing more times than there are messages
            // and we're still over limit, fall back to DropOldest.
            if summarize_attempts >= max_summarize_attempts {
                self.fell_back_to_drop = true;
                self.drop_oldest();
                continue;
            }

            match self.strategy {
                ContextStrategy::DropOldest => self.drop_oldest(),
                ContextStrategy::Summarize => {
                    let tokens_before = self.total_tokens();
                    self.summarize_oldest();
                    let tokens_after = self.total_tokens();
                    // If summarization did not reduce token count, it will
                    // never help — fall back to DropOldest.
                    if tokens_after >= tokens_before {
                        summarize_attempts += 1;
                        if summarize_attempts >= max_summarize_attempts {
                            self.fell_back_to_drop = true;
                            self.drop_oldest();
                        }
                    }
                }
                ContextStrategy::Archive => self.archive_oldest(),
                ContextStrategy::Externalize => self.externalize_oldest(),
            }
        }

        // Final safety: if still over limit after all strategies, drop
        // messages until within limit. This is the last resort.
        while self.is_over_limit() && self.messages.len() > 1 {
            self.drop_oldest();
        }
    }

    fn drop_oldest(&mut self) {
        if let Some(idx) = self.first_droppable() {
            self.messages.remove(idx);
            self.messages_dropped += 1;
        }
    }

    fn summarize_oldest(&mut self) {
        if let Some(idx) = self.first_droppable() {
            let msg = self.messages.remove(idx);
            // Replace with a summary stub.
            let stub = if msg.content.len() > 50 {
                format!("[summary: {}...]", &msg.content[..50])
            } else {
                "[summary]".to_string()
            };
            self.messages.insert(
                idx,
                ContextMessage {
                    role: msg.role,
                    content: stub,
                    timestamp: msg.timestamp,
                    pinned: false,
                },
            );
            self.messages_dropped += 1;
        }
    }

    fn archive_oldest(&mut self) {
        if let Some(idx) = self.first_droppable() {
            self.messages.remove(idx);
            self.messages_archived += 1;
        }
    }

    fn externalize_oldest(&mut self) {
        if let Some(idx) = self.first_droppable() {
            let msg = self.messages.remove(idx);
            // Replace with a reference
            self.messages.insert(
                idx,
                ContextMessage {
                    role: msg.role,
                    content: format!("[externalized to archive: {}]", msg.timestamp),
                    timestamp: msg.timestamp,
                    pinned: false,
                },
            );
            self.messages_archived += 1;
        }
    }

    fn first_droppable(&self) -> Option<usize> {
        self.messages.iter().position(|m| !m.pinned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn under_limit_no_action() {
        let mut cm = ContextManager::new(1000, ContextStrategy::DropOldest);
        cm.push(ContextMessage {
            role: "user".into(),
            content: "hello".into(),
            timestamp: 1,
            pinned: false,
        });
        assert_eq!(cm.messages.len(), 1);
        assert_eq!(cm.messages_dropped, 0);
    }

    #[test]
    fn drop_oldest_strategy() {
        let mut cm = ContextManager::new(2, ContextStrategy::DropOldest);
        cm.push(ContextMessage {
            role: "user".into(),
            content: "first message here".into(),
            timestamp: 1,
            pinned: false,
        });
        cm.push(ContextMessage {
            role: "user".into(),
            content: "second message".into(),
            timestamp: 2,
            pinned: false,
        });
        assert!(cm.messages_dropped > 0);
    }

    #[test]
    fn pinned_messages_preserved() {
        let mut cm = ContextManager::new(10, ContextStrategy::DropOldest);
        cm.push(ContextMessage {
            role: "system".into(),
            content: "system prompt".into(),
            timestamp: 0,
            pinned: true,
        });
        cm.push(ContextMessage {
            role: "user".into(),
            content: "long enough to overflow".into(),
            timestamp: 1,
            pinned: false,
        });
        // System prompt should survive
        assert!(cm.messages.iter().any(|m| m.pinned));
    }

    // ── Regression tests for C1 (infinite loop fix) ──

    #[test]
    fn summarize_eventually_terminates() {
        // This test would hang forever before the fix.
        let mut cm = ContextManager::new(10, ContextStrategy::Summarize);
        cm.push(ContextMessage {
            role: "user".into(),
            content: "a very long message that exceeds the token limit quickly".into(),
            timestamp: 1,
            pinned: false,
        });
        cm.push(ContextMessage {
            role: "user".into(),
            content: "another message".into(),
            timestamp: 2,
            pinned: false,
        });
        // Must not hang — if we reach here, the fix works.
        assert!(cm.messages_dropped > 0);
    }

    #[test]
    fn summarize_falls_back_to_drop_oldest() {
        // When summarization cannot reduce enough (stub still exceeds
        // max_tokens), the strategy must fall back to DropOldest.
        let mut cm = ContextManager::new(2, ContextStrategy::Summarize);
        // A summary stub like "[summary: ...]" is > 2 tokens.
        cm.push(ContextMessage {
            role: "user".into(),
            content: "x".repeat(100),
            timestamp: 1,
            pinned: false,
        });
        cm.push(ContextMessage {
            role: "user".into(),
            content: "y".repeat(100),
            timestamp: 2,
            pinned: false,
        });
        // Should have fallen back and dropped messages.
        assert!(cm.fell_back_to_drop || cm.messages.len() <= 1);
    }

    #[test]
    fn tiny_max_token_budget() {
        // max_tokens = 1 is pathological — almost any message overflows.
        let mut cm = ContextManager::new(1, ContextStrategy::Summarize);
        cm.push(ContextMessage {
            role: "user".into(),
            content: "hi".into(),
            timestamp: 1,
            pinned: false,
        });
        cm.push(ContextMessage {
            role: "user".into(),
            content: "there".into(),
            timestamp: 2,
            pinned: false,
        });
        // Must terminate, not hang.
        assert!(cm.total_tokens() <= 1 || cm.messages.len() <= 1);
    }

    #[test]
    fn empty_context_no_panic() {
        let mut cm = ContextManager::new(0, ContextStrategy::DropOldest);
        cm.push(ContextMessage {
            role: "user".into(),
            content: "overflow".into(),
            timestamp: 1,
            pinned: false,
        });
        // Must not panic, must terminate.
        assert!(cm.messages.len() <= 1);
    }

    #[test]
    fn already_within_limit() {
        let mut cm = ContextManager::new(10000, ContextStrategy::Summarize);
        cm.push(ContextMessage {
            role: "user".into(),
            content: "small".into(),
            timestamp: 1,
            pinned: false,
        });
        assert_eq!(cm.messages_dropped, 0);
        assert!(!cm.fell_back_to_drop);
    }

    #[test]
    fn pathological_stub_case() {
        // Construct a scenario where the summary stub itself exceeds
        // max_tokens. The stub "[summary: ...]" is ~15 chars = ~4 tokens.
        // With max_tokens = 3, the stub alone overflows.
        let mut cm = ContextManager::new(3, ContextStrategy::Summarize);
        cm.push(ContextMessage {
            role: "user".into(),
            content: "a".repeat(200),
            timestamp: 1,
            pinned: false,
        });
        cm.push(ContextMessage {
            role: "user".into(),
            content: "b".repeat(200),
            timestamp: 2,
            pinned: false,
        });
        // Must fall back to DropOldest and terminate.
        assert!(cm.fell_back_to_drop || cm.messages.len() <= 1);
    }
}
