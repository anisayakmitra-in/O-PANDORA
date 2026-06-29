//! Execution Memory Store (ANUBIS integration).
//!
//! Stores execution records, artifacts, and lineage.
//! This is the ANUBIS integration point.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use pandora_types::execution_memory::ExecutionRecord;

/// Stores execution history for ANUBIS.
pub struct MemoryStore {
    records: Arc<Mutex<BTreeMap<String, ExecutionRecord>>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore {
            records: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    /// Store an execution record.
    pub fn store(&self, record: ExecutionRecord) {
        self.records
            .lock()
            .unwrap()
            .insert(record.session_id.clone(), record);
    }

    /// Retrieve an execution record.
    pub fn get(&self, session_id: &str) -> Option<ExecutionRecord> {
        self.records.lock().unwrap().get(session_id).cloned()
    }

    /// List all stored records.
    pub fn list(&self) -> Vec<ExecutionRecord> {
        self.records.lock().unwrap().values().cloned().collect()
    }

    /// Count of stored records.
    pub fn count(&self) -> usize {
        self.records.lock().unwrap().len()
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}
