//! Execution History.
//!
//! Stores workflow graph, execution graph, decision graph,
//! memory graph, capability graph, telemetry, diagnostics,
//! artifacts, branch history, rollback history, repair history.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// History entry kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HistoryKind {
    Workflow,
    Execution,
    Decision,
    Memory,
    Capability,
    Telemetry,
    Diagnostic,
    Artifact,
    Branch,
    Rollback,
    Repair,
}

/// A history entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub entry_id: String,
    pub kind: HistoryKind,
    pub target_id: String,
    pub payload: String,
    pub metadata: BTreeMap<String, String>,
    pub timestamp_ms: u64,
}

/// Execution history store.
pub struct ExecutionHistory {
    entries: Vec<HistoryEntry>,
}

impl ExecutionHistory {
    pub fn new() -> Self {
        ExecutionHistory {
            entries: Vec::new(),
        }
    }

    pub fn record(&mut self, entry: HistoryEntry) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn filter_by_kind(&self, kind: HistoryKind) -> Vec<&HistoryEntry> {
        self.entries.iter().filter(|e| e.kind == kind).collect()
    }
}

impl Default for ExecutionHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_record_and_filter() {
        let mut h = ExecutionHistory::new();
        h.record(HistoryEntry {
            entry_id: "h1".to_string(),
            kind: HistoryKind::Workflow,
            target_id: "wf-1".to_string(),
            payload: "start".to_string(),
            metadata: BTreeMap::new(),
            timestamp_ms: 0,
        });
        h.record(HistoryEntry {
            entry_id: "h2".to_string(),
            kind: HistoryKind::Rollback,
            target_id: "wf-1".to_string(),
            payload: "rollback".to_string(),
            metadata: BTreeMap::new(),
            timestamp_ms: 1,
        });
        assert_eq!(h.entries().len(), 2);
        assert_eq!(h.filter_by_kind(HistoryKind::Rollback).len(), 1);
    }
}
