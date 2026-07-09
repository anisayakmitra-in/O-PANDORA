//! First-class Session model.
//!
//! Every execution is a Session. Sessions tie together prompt, workflow,
//! timeline, telemetry, artifacts, and the ledger. They are persisted
//! in the SessionStore and can be replayed.

use crate::recorder::ExecutionFrame;
use crate::PandoraError;
use std::collections::HashMap;
use std::time::SystemTime;

/// Execution status.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SessionStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

/// A single execution session.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
        self.completed_at
            .map(|end| end.duration_since(self.created_at).unwrap_or_default())
    }

    pub fn add_frame(&mut self, frame: ExecutionFrame) {
        self.timeline.push(frame);
    }

    pub fn add_artifact(&mut self, path: impl Into<String>) {
        self.artifacts.push(path.into());
    }
}

/// In-memory session store with lookup by id, prompt, and status.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SessionStore {
    sessions: HashMap<String, Session>,
}

fn sessions_dir() -> std::path::PathBuf {
    let base = std::env::var("PANDORA_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            std::path::PathBuf::from(home).join(".pandora")
        });
    base.join("sessions")
}

impl SessionStore {
    pub fn new() -> Self {
        let mut store = Self {
            sessions: HashMap::new(),
        };
        // ponytail: load existing sessions silently; don't fail if none
        if let Err(e) = store.load() {
            // first run — no sessions yet
            let _ = e;
        }
        store
    }

    /// Persist all sessions to disk as JSON files.
    pub fn save(&self) -> Result<(), String> {
        let dir = sessions_dir();
        std::fs::create_dir_all(&dir).map_err(|e| format!("Cannot create sessions dir: {}", e))?;
        for session in self.sessions.values() {
            let path = dir.join(format!("{}.json", session.id));
            let json =
                serde_json::to_string_pretty(session).map_err(|e| format!("Serialize: {}", e))?;
            // ponytail: atomic write via tempfile rename
            let tmp = dir.join(format!("{}.tmp", session.id));
            std::fs::write(&tmp, &json).map_err(|e| format!("Write: {}", e))?;
            std::fs::rename(&tmp, &path).map_err(|e| format!("Rename: {}", e))?;
        }
        // Save index (ordered list of session IDs)
        let index: Vec<String> = self.sessions.keys().cloned().collect();
        let index_path = dir.join("index.json");
        std::fs::write(
            &index_path,
            serde_json::to_string_pretty(&index).map_err(|e| format!("Index: {}", e))?,
        )
        .map_err(|e| format!("Write index: {}", e))?;
        Ok(())
    }

    /// Load all sessions from disk.
    pub fn load(&mut self) -> Result<(), String> {
        let dir = sessions_dir();
        if !dir.exists() {
            return Ok(()); // first run
        }
        // Load index first for ordering
        let index_path = dir.join("index.json");
        if index_path.exists() {
            let content =
                std::fs::read_to_string(&index_path).map_err(|e| format!("Read index: {}", e))?;
            let ids: Vec<String> =
                serde_json::from_str(&content).map_err(|e| format!("Parse index: {}", e))?;
            for id in &ids {
                let path = dir.join(format!("{}.json", id));
                if !path.exists() {
                    continue;
                }
                let json =
                    std::fs::read_to_string(&path).map_err(|e| format!("Read {}: {}", id, e))?;
                let session: Session =
                    serde_json::from_str(&json).map_err(|e| format!("Parse {}: {}", id, e))?;
                self.sessions.insert(id.clone(), session);
            }
        } else {
            // No index — scan for .json files
            for entry in std::fs::read_dir(&dir).map_err(|e| format!("Read dir: {}", e))? {
                let entry = entry.map_err(|e| format!("Entry: {}", e))?;
                let path = entry.path();
                if path.extension().is_none_or(|e| e != "json")
                    || path.file_stem() == Some(std::ffi::OsStr::new("index"))
                {
                    continue;
                }
                let json = std::fs::read_to_string(&path).map_err(|e| format!("Read: {}", e))?;
                if let Ok(session) = serde_json::from_str::<Session>(&json) {
                    self.sessions.insert(session.id.clone(), session);
                }
            }
        }
        Ok(())
    }

    pub fn create(&mut self, id: impl Into<String>, prompt: impl Into<String>) -> &mut Session {
        let id = id.into();
        let prompt = prompt.into();
        if !self.sessions.contains_key(&id) {
            self.sessions
                .insert(id.clone(), Session::new(id.clone(), prompt));
            // ponytail: persist immediately so sessions survive crashes
            let _ = self.save();
        }
        self.sessions.get_mut(&id).unwrap()
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
        self.sessions
            .values()
            .filter(|s| s.status == *status)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Session> {
        let q = query.to_lowercase();
        self.sessions
            .values()
            .filter(|s| s.prompt.to_lowercase().contains(&q) || s.id.contains(&q))
            .collect()
    }

    pub fn remove(&mut self, id: &str) -> Result<(), PandoraError> {
        self.sessions
            .remove(id)
            .ok_or_else(|| PandoraError::not_found(format!("Session not found: {}", id)))?;
        let _ = self.save();
        // Also remove the file
        let path = sessions_dir().join(format!("{}.json", id));
        let _ = std::fs::remove_file(&path);
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.sessions.len()
    }

    /// Replay a session by creating a new session seeded with the originals timeline.
    pub fn replay(&mut self, id: &str) -> Result<Session, PandoraError> {
        let original = self
            .get(id)
            .ok_or_else(|| PandoraError::not_found(format!("Session not found: {}", id)))?;
        let mut replayed = Session::new(
            format!("replay-{}", id),
            format!("[REPLAY] {}", original.prompt),
        );
        replayed
            .metadata
            .insert("original_session".to_string(), id.to_string());
        replayed.replay_id = original.replay_id.clone();
        Ok(replayed)
    }
}
