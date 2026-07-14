//! Execution ledger — records every stage decision with full provenance.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LedgerOutcome { Success, Failure(String) }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub execution_id: String, pub stage: String, pub decision: String,
    pub duration_ms: u64, pub timestamp: String, pub skill_version: Option<String>,
    pub outcome: LedgerOutcome, pub previous_hash: Option<String>, pub hash: String,
    pub metadata: HashMap<String, String>, pub cost: f64, pub reason: String,
    pub workflow: String, pub provider: String, pub entry_id: String,
}

impl Default for LedgerEntry {
    fn default() -> Self {
        Self { execution_id: String::new(), stage: String::new(), decision: String::new(),
            duration_ms: 0, timestamp: String::new(), skill_version: None, outcome: LedgerOutcome::Success,
            previous_hash: None, hash: String::new(), metadata: HashMap::new(),
            cost: 0.0, reason: String::new(), workflow: String::new(), provider: String::new(), entry_id: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLedger { pub entries: Vec<LedgerEntry> }

impl ExecutionLedger {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn append(&mut self, entry: LedgerEntry) -> &mut Self { self.entries.push(entry); self }
    pub fn len(&self) -> usize { self.entries.len() }
}
