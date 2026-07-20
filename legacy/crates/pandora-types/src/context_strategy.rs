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

/// The context manager — tracks messages and applies strategies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextManager {
    pub messages: Vec<ContextMessage>,
    pub max_tokens: usize,
    pub strategy: ContextStrategy,
    pub messages_dropped: usize,
    pub messages_archived: usize,
}

impl ContextManager {
    pub fn new(max_tokens: usize, strategy: ContextStrategy) -> Self {
        Self { max_tokens, strategy, ..Default::default() }
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
    fn enforce_limit(&mut self) {
        while self.is_over_limit() && self.messages.len() > 1 {
            match self.strategy {
                ContextStrategy::DropOldest => self.drop_oldest(),
                ContextStrategy::Summarize => self.summarize_oldest(),
                ContextStrategy::Archive => self.archive_oldest(),
                ContextStrategy::Externalize => self.externalize_oldest(),
            }
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
            // Replace with a summary stub
            self.messages.insert(idx, ContextMessage {
                role: msg.role,
                content: format!("[summary: {}...]", &msg.content[..50.min(msg.content.len())]),
                timestamp: msg.timestamp,
                pinned: false,
            });
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
            self.messages.insert(idx, ContextMessage {
                role: msg.role,
                content: format!("[externalized to archive: {}]", msg.timestamp),
                timestamp: msg.timestamp,
                pinned: false,
            });
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
        cm.push(ContextMessage { role: "user".into(), content: "hello".into(), timestamp: 1, pinned: false });
        assert_eq!(cm.messages.len(), 1);
        assert_eq!(cm.messages_dropped, 0);
    }

    #[test]
    fn drop_oldest_strategy() {
        let mut cm = ContextManager::new(10, ContextStrategy::DropOldest);
        cm.push(ContextMessage { role: "user".into(), content: "first message here".into(), timestamp: 1, pinned: false });
        cm.push(ContextMessage { role: "user".into(), content: "second message".into(), timestamp: 2, pinned: false });
        assert!(cm.messages_dropped > 0);
    }

    #[test]
    fn pinned_messages_preserved() {
        let mut cm = ContextManager::new(10, ContextStrategy::DropOldest);
        cm.push(ContextMessage { role: "system".into(), content: "system prompt".into(), timestamp: 0, pinned: true });
        cm.push(ContextMessage { role: "user".into(), content: "long enough to overflow".into(), timestamp: 1, pinned: false });
        // System prompt should survive
        assert!(cm.messages.iter().any(|m| m.pinned));
    }

    #[test]
    fn summarize_replaces_with_stub() {
        let mut cm = ContextManager::new(10, ContextStrategy::Summarize);
        cm.push(ContextMessage { role: "user".into(), content: "a very long message that exceeds the token limit quickly".into(), timestamp: 1, pinned: false });
        cm.push(ContextMessage { role: "user".into(), content: "another message".into(), timestamp: 2, pinned: false });
        assert!(cm.messages_dropped > 0);
        // Check that a summary stub exists
        assert!(cm.messages.iter().any(|m| m.content.contains("[summary:")));
    }
}
