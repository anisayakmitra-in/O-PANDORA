//! Phoenix Execution Manager.
//!
//! Phoenix is the sole execution runtime. This manager
//! creates sessions, runs execution graphs, tracks
//! checkpoints, and handles rollback/recovery.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use pandora_types::execution::{
    ExecutionBudget, ExecutionContext, ExecutionResult, ExecutionSession, ExecutionStatistics,
    ExecutionStatus,
};
use pandora_types::universal::Health;

/// Manages execution sessions for Phoenix.
pub struct ExecutionManager {
    sessions: Arc<Mutex<BTreeMap<String, ExecutionSession>>>,
    results: Arc<Mutex<BTreeMap<String, ExecutionResult>>>,
    next_id: Arc<Mutex<u64>>,
}

impl ExecutionManager {
    pub fn new() -> Self {
        ExecutionManager {
            sessions: Arc::new(Mutex::new(BTreeMap::new())),
            results: Arc::new(Mutex::new(BTreeMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new execution session.
    pub fn create_session(
        &self,
        context: ExecutionContext,
        budget: ExecutionBudget,
    ) -> ExecutionSession {
        let id = self.next_id();
        let session = ExecutionSession {
            session_id: format!("exec-{}", id),
            status: ExecutionStatus::Pending,
            health: Health::Healthy,
            lifecycle: pandora_types::universal::Lifecycle::Created,
            created_at_ms: 0,
            budget,
            context,
        };
        self.sessions
            .lock()
            .unwrap()
            .insert(session.session_id.clone(), session.clone());
        session
    }

    /// Mark a session as running.
    pub fn start(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(s) = sessions.get_mut(session_id) {
            s.status = ExecutionStatus::Running;
            s.lifecycle = pandora_types::universal::Lifecycle::Running;
            true
        } else {
            false
        }
    }

    /// Complete a session with a result.
    pub fn complete(
        &self,
        session_id: &str,
        output: Option<String>,
        error: Option<String>,
        duration_ms: u64,
    ) -> bool {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(s) = sessions.get_mut(session_id) {
            let status = if error.is_some() {
                ExecutionStatus::Failed
            } else {
                ExecutionStatus::Completed
            };
            s.status = status;
            s.lifecycle = pandora_types::universal::Lifecycle::Stopped;

            let result = ExecutionResult {
                session_id: session_id.to_string(),
                status,
                output,
                error,
                duration_ms,
                cost_cents: 0,
            };
            self.results
                .lock()
                .unwrap()
                .insert(session_id.to_string(), result);
            true
        } else {
            false
        }
    }

    /// Get a session by ID.
    pub fn get_session(&self, session_id: &str) -> Option<ExecutionSession> {
        self.sessions.lock().unwrap().get(session_id).cloned()
    }

    /// Get a result by session ID.
    pub fn get_result(&self, session_id: &str) -> Option<ExecutionResult> {
        self.results.lock().unwrap().get(session_id).cloned()
    }

    /// List all active sessions.
    pub fn active_sessions(&self) -> Vec<ExecutionSession> {
        self.sessions
            .lock()
            .unwrap()
            .values()
            .filter(|s| s.status == ExecutionStatus::Running)
            .cloned()
            .collect()
    }

    /// Statistics.
    pub fn statistics(&self) -> ExecutionStatistics {
        let sessions = self.sessions.lock().unwrap();
        let results = self.results.lock().unwrap();
        ExecutionStatistics {
            total_executions: sessions.len() as u64,
            active_executions: sessions
                .values()
                .filter(|s| s.status == ExecutionStatus::Running)
                .count() as u64,
            completed_executions: results
                .values()
                .filter(|r| r.status == ExecutionStatus::Completed)
                .count() as u64,
            failed_executions: results
                .values()
                .filter(|r| r.status == ExecutionStatus::Failed)
                .count() as u64,
            avg_duration_ms: if results.is_empty() {
                0
            } else {
                results.values().map(|r| r.duration_ms).sum::<u64>() / results.len() as u64
            },
            total_cost_cents: results.values().map(|r| r.cost_cents).sum(),
        }
    }

    fn next_id(&self) -> u64 {
        let mut id = self.next_id.lock().unwrap();
        let current = *id;
        *id += 1;
        current
    }
}

impl Default for ExecutionManager {
    fn default() -> Self {
        Self::new()
    }
}
