//! Pandora Execution Ledger — append-only immutable execution log.
//!
//! Records every execution decision, outcome, and context as an immutable
//! entry. Queryable by Parliament for:
//!   - "Why did we stop using Provider X?"
//!   - "When did Ollama become preferred?"
//!   - "Why was Gene Y quarantined?"
//!
//! Different from Recorder (execution frames) and Replay (deterministic replay).
//! The Ledger stores WHY decisions were made, not just WHAT happened.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An immutable entry in the execution ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// Unique execution identifier.
    pub execution_id: String,
    /// When this execution occurred.
    pub timestamp: String,
    /// Which provider was used.
    pub provider: String,
    /// Which workflow was executed.
    pub workflow: String,
    /// Which skill version was used (if applicable).
    pub skill_version: Option<String>,
    /// Why this execution happened.
    pub reason: String,
    /// Cost in tokens, money, or compute units.
    pub cost: f64,
    /// What decision was made (e.g., provider selection, gene activation).
    pub decision: String,
    /// Outcome: success, failure, partial, cancelled.
    pub outcome: LedgerOutcome,
    /// Content-addressable hash linking to previous entry.
    pub previous_hash: Option<String>,
    /// Current entry's hash.
    pub hash: String,
    /// Arbitrary metadata.
    pub metadata: HashMap<String, String>,
}

/// Outcome of an execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LedgerOutcome {
    Success,
    Failure(String),
    Partial,
    Cancelled,
}

/// Append-only ledger of execution decisions.
pub struct ExecutionLedger {
    entries: Vec<LedgerEntry>,
    max_entries: usize,
}

impl ExecutionLedger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 100_000,
        }
    }

    pub fn with_max(max: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries: max,
        }
    }

    /// Append a new entry to the ledger.
    pub fn append(&mut self, entry: LedgerEntry) -> Result<(), String> {
        if self.entries.len() >= self.max_entries {
            return Err("Ledger full".into());
        }
        self.entries.push(entry);
        Ok(())
    }

    /// Query entries by provider.
    pub fn by_provider(&self, provider: &str) -> Vec<&LedgerEntry> {
        self.entries
            .iter()
            .filter(|e| e.provider == provider)
            .collect()
    }

    /// Query entries by outcome.
    pub fn by_outcome(&self, outcome: &LedgerOutcome) -> Vec<&LedgerEntry> {
        self.entries
            .iter()
            .filter(|e| std::mem::discriminant(&e.outcome) == std::mem::discriminant(outcome))
            .collect()
    }

    /// Query entries within a time range.
    pub fn by_time_range(&self, start: &str, end: &str) -> Vec<&LedgerEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start.to_string() && e.timestamp <= end.to_string())
            .collect()
    }

    /// Get the most recent N entries.
    pub fn recent(&self, n: usize) -> Vec<&LedgerEntry> {
        self.entries.iter().rev().take(n).collect()
    }

    /// Search entries by reason or decision text.
    pub fn search(&self, query: &str) -> Vec<&LedgerEntry> {
        let q = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.reason.to_lowercase().contains(&q) || e.decision.to_lowercase().contains(&q)
            })
            .collect()
    }

    /// Total number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Export all entries as JSON.
    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&self.entries).unwrap_or_default()
    }
}

impl Default for ExecutionLedger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entry(id: &str) -> LedgerEntry {
        LedgerEntry {
            execution_id: id.into(),
            timestamp: "2026-07-03T12:00:00Z".into(),
            provider: "ollama".into(),
            workflow: "code-review".into(),
            skill_version: None,
            reason: "Code review requested".into(),
            cost: 0.05,
            decision: "Selected ollama/qwen2.5-coder".into(),
            outcome: LedgerOutcome::Success,
            previous_hash: None,
            hash: format!("hash-{}", id),
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn append_and_query() {
        let mut ledger = ExecutionLedger::new();
        ledger.append(sample_entry("1")).unwrap();
        ledger.append(sample_entry("2")).unwrap();
        assert_eq!(ledger.len(), 2);
        assert_eq!(ledger.by_provider("ollama").len(), 2);
    }

    #[test]
    fn search_by_text() {
        let mut ledger = ExecutionLedger::new();
        ledger.append(sample_entry("1")).unwrap();
        let results = ledger.search("review");
        assert!(!results.is_empty());
    }

    #[test]
    fn empty_ledger() {
        let ledger = ExecutionLedger::new();
        assert!(ledger.is_empty());
        assert!(ledger.recent(10).is_empty());
    }
}
