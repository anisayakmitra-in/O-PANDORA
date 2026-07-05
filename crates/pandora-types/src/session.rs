//! First-class Session model.
//!
//! Every execution is a Session. Sessions tie together prompt, workflow,
//! timeline, telemetry, artifacts, and the ledger. They are persisted
//! in the SessionStore and can be replayed.

use crate::PandoraError;
use crate::recorder::ExecutionFrame;
use std::collections::HashMap;
use std::time::SystemTime;

/// Execution status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

/// A single execution session.
#[derive(Debug, Clone)]
pub struct Session {
    /// Unique session identifier.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// The original prompt.
    pub prompt: String,
    /// When the session was created.
    pub created_at: SystemTime,
    /// When the session completed (if it has).
    pub completed_at: Option<SystemTime>,
    /// Current status.
    pub status: SessionStatus,
    /// Selected workflow (optional).
    pub workflow: Option<String>,
    /// Execution timeline (ordered frames).
    pub timeline: Vec<ExecutionFrame>,

    /// Ledger entries.
    pub ledger: Vec<String>,
    /// Artifact paths produced during execution.
    pub artifacts: Vec<String>,
    /// Key-value metadata.
    pub metadata: HashMap<String, String>,
    /// Replay identifier.
    pub replay_id: Option<String>,
}

impl Session {
    pub fn new(id: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: String::new(),
            prompt: prompt.into(),
            created_at: SystemTime::now(),
            completed_at: None,
            status: SessionStatus::Pending,
            workflow: None,
            timeline: Vec::new(),
            
            ledger: Vec::new(),
            artifacts: Vec::new(),
            metadata: HashMap::new(),
            replay_id: None,
        }
    }

    pub fn duration(&self) -> Option<std::time::Duration> {
        self.completed_at.map(|end| end.duration_since(self.created_at).unwrap_or_default())
    }

    pub fn add_frame(&mut self, frame: ExecutionFrame) {
        self.timeline.push(frame);
    }



    pub fn add_artifact(&mut self, path: impl Into<String>) {
        self.artifacts.push(path.into());
    }
}

/// In-memory session store with lookup by id, prompt, and status.
#[derive(Debug, Default)]
pub struct SessionStore {
    sessions: HashMap<String, Session>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self { sessions: HashMap::new() }
    }

    pub fn create(&mut self, id: impl Into<String>, prompt: impl Into<String>) -> &mut Session {
        let id = id.into();
        let prompt = prompt.into();
        self.sessions.entry(id.clone()).or_insert_with(|| Session::new(id, prompt))
    }

    pub fn get(&self, id: &str) -> Option<&Session> {
        self.sessions.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    }

    pub fn all(&self) -> Vec<&Session> {
        let mut v: Vec<&Session> = self.sessions.values().collect();
        v.sort_by_key(|s| s.created_at);
        v.reverse();
        v
    }

    pub fn recent(&self, n: usize) -> Vec<&Session> {
        self.all().into_iter().take(n).collect()
    }

    pub fn by_status(&self, status: &SessionStatus) -> Vec<&Session> {
        self.sessions.values().filter(|s| s.status == *status).collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Session> {
        let q = query.to_lowercase();
        self.sessions.values()
            .filter(|s| s.prompt.to_lowercase().contains(&q) || s.id.contains(&q))
            .collect()
    }

    pub fn remove(&mut self, id: &str) -> Result<(), PandoraError> {
        self.sessions.remove(id).ok_or_else(|| PandoraError::not_found(format!("Session not found: {}", id)))?;
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.sessions.len()
    }

    /// Replay a session by creating a new session seeded with the originals timeline.
    pub fn replay(&mut self, id: &str) -> Result<Session, PandoraError> {
        let original = self.get(id).ok_or_else(|| PandoraError::not_found(format!("Session not found: {}", id)))?;
        let mut replayed = Session::new(
            format!("replay-{}", id),
            format!("[REPLAY] {}", original.prompt),
        );
        replayed.metadata.insert("original_session".to_string(), id.to_string());
        replayed.replay_id = original.replay_id.clone();
        Ok(replayed)
    }
}
