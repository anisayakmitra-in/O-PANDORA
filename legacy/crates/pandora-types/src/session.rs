//! Session model — every execution is a Session.
//!
//! Sessions tie together prompt, workflow, timeline, telemetry, artifacts,
//! and the ledger. They are persisted in the `SessionStore` and can be
//! replayed, inspected, and exported.

use crate::recorder::ExecutionFrame;
use crate::PandoraError;
use std::collections::HashMap;
use std::time::SystemTime;

#[non_exhaustive]
/// Execution status.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SessionStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

/// A single execution session — the primary record of what happened.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Session {
    /// Unique session identifier.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// The original prompt that started this execution.
    pub prompt: String,
    /// When the session was created.
    pub created_at: SystemTime,
    /// When the session completed (if it has).
    pub completed_at: Option<SystemTime>,
    /// Current status.
    pub status: SessionStatus,
    /// Selected workflow (optional).
    pub workflow: Option<String>,
    /// Execution timeline as ordered frames.
    pub timeline: Vec<ExecutionFrame>,
    /// Ledger entries recorded during execution.
    pub ledger: Vec<String>,
    /// Artifact paths produced during execution.
    pub artifacts: Vec<String>,
    /// Key-value metadata for extensibility.
    pub metadata: HashMap<String, String>,
    /// Replay identifier, if this session was replayed.
    pub replay_id: Option<String>,
}

impl Session {
    /// Create a new pending session.
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

    /// Duration from creation to completion, if completed.
    pub fn duration(&self) -> Option<std::time::Duration> {
        self.completed_at
            .map(|end| end.duration_since(self.created_at).unwrap_or_default())
    }

    /// Append a frame to the execution timeline.
    pub fn add_frame(&mut self, frame: ExecutionFrame) {
        self.timeline.push(frame);
    }

    /// Record an artifact path.
    pub fn add_artifact(&mut self, path: impl Into<String>) {
        self.artifacts.push(path.into());
    }
}

/// In-memory session store with JSON file persistence.
///
/// Sessions are cached in a `HashMap` and atomically written to individual
/// JSON files in the sessions directory on every mutation.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SessionStore {
    sessions: HashMap<String, Session>,
}

// ── Helpers ──

fn is_reserved_windows_device_name(id: &str) -> bool {
    let upper = id.to_ascii_uppercase();
    matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (upper.len() == 4
            && matches!(&upper[..3], "COM" | "LPT")
            && matches!(upper.as_bytes()[3], b'1'..=b'9'))
}

fn validate_session_id(id: &str) -> Result<(), PandoraError> {
    if id.is_empty()
        || id.len() > 128
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        || is_reserved_windows_device_name(id)
    {
        return Err(PandoraError::validation("invalid session ID"));
    }
    Ok(())
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

fn write_sessions(sessions: &HashMap<String, Session>) -> Result<(), crate::PandoraError> {
    for (id, session) in sessions {
        validate_session_id(id)?;
        validate_session_id(&session.id)?;
        if id != &session.id {
            return Err(PandoraError::validation(
                "session key does not match session ID",
            ));
        }
    }
    let dir = sessions_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| crate::PandoraError::Internal(format!("Cannot create sessions dir: {e}")))?;
    for session in sessions.values() {
        let path = dir.join(format!("{}.json", session.id));
        let json = serde_json::to_string_pretty(session).map_err(|e| {
            crate::PandoraError::Internal(format!("Serialize session {}: {e}", session.id))
        })?;
        let tmp = dir.join(format!("{}.tmp", session.id));
        std::fs::write(&tmp, &json).map_err(|e| {
            crate::PandoraError::Internal(format!("Write session {}: {e}", session.id))
        })?;
        std::fs::rename(&tmp, &path).map_err(|e| {
            crate::PandoraError::Internal(format!("Rename session {}: {e}", session.id))
        })?;
    }
    let index: Vec<String> = sessions.keys().cloned().collect();
    let index_path = dir.join("index.json");
    std::fs::write(
        &index_path,
        serde_json::to_string_pretty(&index)
            .map_err(|e| crate::PandoraError::Internal(format!("Serialize index: {e}")))?,
    )
    .map_err(|e| crate::PandoraError::Internal(format!("Write index: {e}")))
}

// ── SessionStore implementations ──

impl SessionStore {
    /// Create a new store and load existing sessions from disk.
    pub fn new() -> Self {
        let mut store = Self {
            sessions: HashMap::new(),
        };
        // ponytail: load existing sessions silently; first-run has no sessions
        if let Err(e) = store.load() {
            let _ = e;
        }
        store
    }

    /// Persist all sessions to disk as atomic JSON writes.
    pub fn save(&self) -> Result<(), crate::PandoraError> {
        write_sessions(&self.sessions)
    }

    /// Load all sessions from disk.
    ///
    /// First tries `index.json` for ordering, then falls back to a
    /// directory scan of `.json` files.
    pub fn load(&mut self) -> Result<(), crate::PandoraError> {
        let dir = sessions_dir();
        if !dir.exists() {
            return Ok(());
        }
        let index_path = dir.join("index.json");
        if index_path.exists() {
            let content = std::fs::read_to_string(&index_path)
                .map_err(|e| crate::PandoraError::Internal(format!("Read index: {e}")))?;
            let ids: Vec<String> = serde_json::from_str(&content)
                .map_err(|e| crate::PandoraError::Internal(format!("Parse index: {e}")))?;
            for id in &ids {
                validate_session_id(id)?;
                let path = dir.join(format!("{id}.json"));
                if !path.exists() {
                    continue;
                }
                let json = std::fs::read_to_string(&path).map_err(|e| {
                    crate::PandoraError::Internal(format!("Read session {id}: {e}"))
                })?;
                let session: Session = serde_json::from_str(&json).map_err(|e| {
                    crate::PandoraError::Internal(format!("Parse session {id}: {e}"))
                })?;
                self.sessions.insert(id.clone(), session);
            }
        } else {
            for entry in std::fs::read_dir(&dir)
                .map_err(|e| crate::PandoraError::Internal(format!("Read sessions dir: {e}")))?
            {
                let entry =
                    entry.map_err(|e| crate::PandoraError::Internal(format!("Entry: {e}")))?;
                let path = entry.path();
                let is_json = path.extension().is_some_and(|e| e == "json");
                let is_index = path.file_stem() == Some(std::ffi::OsStr::new("index"));
                if !is_json || is_index {
                    continue;
                }
                let json = std::fs::read_to_string(&path)
                    .map_err(|e| crate::PandoraError::Internal(format!("Read: {e}")))?;
                if let Ok(session) = serde_json::from_str::<Session>(&json) {
                    validate_session_id(&session.id)?;
                    self.sessions.insert(session.id.clone(), session);
                }
            }
        }
        Ok(())
    }

    /// Get or create a session by id.
    pub fn create(&mut self, id: impl Into<String>, prompt: impl Into<String>) -> &mut Session {
        let id = id.into();
        let prompt = prompt.into();
        if !self.sessions.contains_key(&id) {
            self.sessions
                .insert(id.clone(), Session::new(id.clone(), prompt));
            // ponytail: persist immediately so sessions survive crashes
            let _ = self.save();
        }
        self.sessions.get_mut(&id).expect("session store")
    }

    /// Get a session by id.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Session> {
        self.sessions.get(id)
    }

    /// Get a mutable session by id.
    #[must_use]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    }

    /// All sessions sorted by creation time (newest first).
    #[must_use]
    pub fn all(&self) -> Vec<&Session> {
        let mut v: Vec<&Session> = self.sessions.values().collect();
        v.sort_by_key(|s| s.created_at);
        v.reverse();
        v
    }

    /// The `n` most recent sessions.
    #[must_use]
    pub fn recent(&self, n: usize) -> Vec<&Session> {
        self.all().into_iter().take(n).collect()
    }

    /// Sessions filtered by status.
    #[must_use]
    pub fn by_status(&self, status: &SessionStatus) -> Vec<&Session> {
        self.sessions
            .values()
            .filter(|s| s.status == *status)
            .collect()
    }

    /// Sessions matching a search query (prompt or id, case-insensitive).
    #[must_use]
    pub fn search(&self, query: &str) -> Vec<&Session> {
        let q = query.to_lowercase();
        self.sessions
            .values()
            .filter(|s| s.prompt.to_lowercase().contains(&q) || s.id.contains(&q))
            .collect()
    }

    /// Remove a session from the store and delete its file.
    pub fn remove(&mut self, id: &str) -> Result<(), PandoraError> {
        validate_session_id(id)?;
        self.sessions
            .remove(id)
            .ok_or_else(|| PandoraError::not_found(format!("Session not found: {id}")))?;
        let _ = self.save();
        let path = sessions_dir().join(format!("{id}.json"));
        let _ = std::fs::remove_file(&path);
        Ok(())
    }

    /// Total number of sessions.
    #[must_use]
    pub fn count(&self) -> usize {
        self.sessions.len()
    }

    /// Create a replay session from an existing session.
    pub fn replay(&mut self, id: &str) -> Result<Session, PandoraError> {
        let original = self
            .get(id)
            .ok_or_else(|| PandoraError::not_found(format!("Session not found: {id}")))?;
        let mut replayed = Session::new(
            format!("replay-{id}"),
            format!("[REPLAY] {}", original.prompt),
        );
        replayed
            .metadata
            .insert("original_session".to_string(), id.to_string());
        replayed.replay_id = original.replay_id.clone();
        Ok(replayed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_home(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("pandora-session-{label}-{}", rand::random::<u64>()))
    }

    #[test]
    fn save_does_not_escape_sessions_directory() {
        let _lock = crate::environment_lock();
        let home = test_home("save");
        let _home = crate::EnvVarGuard::set("PANDORA_HOME", &home);
        let mut store = SessionStore::default();

        store.create("../outside", "unsafe session");

        assert!(!home.join("outside.json").exists());
        let _ = std::fs::remove_dir_all(home);
    }

    #[test]
    fn load_rejects_unsafe_index_ids() {
        let _lock = crate::environment_lock();
        let home = test_home("load");
        let _home = crate::EnvVarGuard::set("PANDORA_HOME", &home);
        let dir = home.join("sessions");
        std::fs::create_dir_all(&dir).expect("create sessions directory");
        std::fs::write(dir.join("index.json"), r#"["../outside"]"#).expect("write index");
        std::fs::write(
            home.join("outside.json"),
            serde_json::to_string(&Session::new("../outside", "unsafe session"))
                .expect("serialize session"),
        )
        .expect("write escaped session");
        let mut store = SessionStore::default();

        assert!(store.load().is_err());
        let _ = std::fs::remove_dir_all(home);
    }

    #[test]
    fn remove_rejects_unsafe_ids_without_deleting_files() {
        let _lock = crate::environment_lock();
        let home = test_home("remove");
        let _home = crate::EnvVarGuard::set("PANDORA_HOME", &home);
        std::fs::create_dir_all(home.join("sessions")).expect("create sessions directory");
        let victim = home.join("victim.json");
        std::fs::write(&victim, "preserve").expect("write victim");
        let mut store = SessionStore::default();
        store.sessions.insert(
            "../victim".into(),
            Session::new("../victim", "unsafe session"),
        );

        assert!(store.remove("../victim").is_err());
        assert!(victim.exists());
        let _ = std::fs::remove_dir_all(home);
    }
}
