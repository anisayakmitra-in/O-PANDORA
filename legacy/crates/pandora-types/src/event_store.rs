//! Event Store — persists PipelineEvent stream for replay and analytics.
//!
//! Every execution records events to a durable log. Replay reads the log
//! and reconstructs the execution state. Analytics queries the log without
//! running the pipeline again.
//!
//! Storage: JSON-line file (`~/.pandora/events/<session-id>.events.json`).

use crate::events::PipelineEvent;
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

/// An appended-event log for execution events.
/// Thread-safe. Each session has its own event file.
#[derive(Debug)]
pub struct EventStore {
    /// Directory where event files are stored.
    dir: PathBuf,
    /// Internal buffer for pending writes.
    buffer: Mutex<Vec<(String, PipelineEvent)>>,
}

fn is_reserved_windows_device_name(session_id: &str) -> bool {
    matches!(
        session_id.to_ascii_uppercase().as_str(),
        "CON"
            | "PRN"
            | "AUX"
            | "NUL"
            | "COM1"
            | "COM2"
            | "COM3"
            | "COM4"
            | "COM5"
            | "COM6"
            | "COM7"
            | "COM8"
            | "COM9"
            | "LPT1"
            | "LPT2"
            | "LPT3"
            | "LPT4"
            | "LPT5"
            | "LPT6"
            | "LPT7"
            | "LPT8"
            | "LPT9"
    )
}

impl EventStore {
    /// Create a new event store rooted at `base_dir/events/`.
    pub fn new(base_dir: PathBuf) -> Self {
        let dir = base_dir.join("events");
        let _ = fs::create_dir_all(&dir);
        Self {
            dir,
            buffer: Mutex::new(Vec::new()),
        }
    }

    fn event_path(&self, session_id: &str) -> Result<PathBuf, crate::PandoraError> {
        if session_id.is_empty()
            || session_id.len() > 128
            || !session_id.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_')
            })
            || is_reserved_windows_device_name(session_id)
        {
            return Err(crate::PandoraError::validation("invalid event session ID"));
        }
        Ok(self.dir.join(format!("{session_id}.events.json")))
    }

    /// Append an event to the in-memory buffer.
    pub fn push(&self, session_id: &str, event: PipelineEvent) {
        let _ = self.try_push(session_id, event);
    }

    /// Append an event after validating its session ID.
    pub fn try_push(
        &self,
        session_id: &str,
        event: PipelineEvent,
    ) -> Result<(), crate::PandoraError> {
        self.event_path(session_id)?;
        let mut buf = self
            .buffer
            .lock()
            .map_err(|e| crate::PandoraError::Internal(e.to_string()))?;
        buf.push((session_id.to_string(), event));
        Ok(())
    }

    /// Flush all buffered events to disk.
    pub fn flush(&self) -> Result<(), crate::PandoraError> {
        let mut buf = self
            .buffer
            .lock()
            .map_err(|e| crate::PandoraError::Internal(e.to_string()))?;
        let events = buf.clone();
        for (session_id, _) in &events {
            self.event_path(session_id)?;
        }
        let mut written = 0;
        let result = (|| -> Result<(), crate::PandoraError> {
            for (session_id, event) in &events {
                let path = self.event_path(session_id)?;
                let line = serde_json::to_string(event)
                    .map_err(|e| crate::PandoraError::Internal(e.to_string()))?;
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .map_err(|e| {
                        crate::PandoraError::Internal(format!("Cannot open event file: {e}"))
                    })?;
                use std::io::Write;
                writeln!(file, "{}", line).map_err(|e| {
                    crate::PandoraError::Internal(format!("Cannot write event: {e}"))
                })?;
                file.sync_data().map_err(|e| {
                    crate::PandoraError::Internal(format!("Cannot sync event: {e}"))
                })?;
                written += 1;
            }
            Ok(())
        })();
        buf.drain(..written);
        result
    }

    /// Read all events for a session, in order.
    pub fn read_events(&self, session_id: &str) -> Result<Vec<PipelineEvent>, crate::PandoraError> {
        let path = self.event_path(session_id)?;
        let content = fs::read_to_string(&path)
            .map_err(|e| crate::PandoraError::Internal(format!("Cannot read events: {e}")))?;
        content
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                serde_json::from_str(l)
                    .map_err(|e| crate::PandoraError::Internal(format!("Parse error: {e}")))
            })
            .collect()
    }

    /// Reconstruct an execution timeline from events.
    pub fn reconstruct_timeline(
        &self,
        session_id: &str,
    ) -> Result<Vec<String>, crate::PandoraError> {
        let events = self.read_events(session_id)?;
        let mut timeline = Vec::new();
        for event in &events {
            match event {
                PipelineEvent::StageStarted { stage } => {
                    timeline.push(format!("Stage started: {stage}"))
                }
                PipelineEvent::StageFinished {
                    stage,
                    success,
                    duration_ms,
                } => timeline.push(format!(
                    "Stage {}: {} ({}ms)",
                    stage,
                    if *success { "OK" } else { "FAIL" },
                    duration_ms
                )),
                PipelineEvent::HarnessSelected { harness, reason } => {
                    timeline.push(format!("Harness: {harness} ({reason})"))
                }
                PipelineEvent::ProviderSelected {
                    provider,
                    model,
                    reason,
                } => timeline.push(format!("Provider: {provider}/{model} ({reason})")),
                PipelineEvent::GeneExecuted {
                    gene,
                    duration_ms,
                    success,
                } => timeline.push(format!(
                    "Gene: {gene} ({}ms, {})",
                    duration_ms,
                    if *success { "OK" } else { "FAIL" }
                )),
                PipelineEvent::DecisionMade { stage, chosen, .. } => {
                    timeline.push(format!("Decision at {stage}: {chosen}"))
                }
                PipelineEvent::EvaluationPassed { evaluator, .. } => {
                    timeline.push(format!("Evaluator passed: {evaluator}"))
                }
                PipelineEvent::EvaluationFailed {
                    evaluator, reason, ..
                } => timeline.push(format!("Evaluator failed: {evaluator} — {reason}")),
                _ => {}
            }
        }
        Ok(timeline)
    }

    /// Count events for a session.
    pub fn event_count(&self, session_id: &str) -> Result<usize, crate::PandoraError> {
        self.read_events(session_id).map(|e| e.len())
    }

    /// List all sessions with event files.
    pub fn list_sessions(&self) -> Vec<String> {
        let mut sessions = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.path().file_stem() {
                    if let Some(s) = name.to_str() {
                        sessions.push(s.trim_end_matches(".events").to_string());
                    }
                }
            }
        }
        sessions.sort();
        sessions.dedup();
        sessions
    }
}

impl Default for EventStore {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        Self::new(PathBuf::from(home).join(".pandora"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::PipelineEvent;

    #[test]
    fn push_and_flush() {
        let dir = std::env::temp_dir().join(format!("evtest-{}", rand::random::<u64>()));
        let store = EventStore::new(dir.clone());
        store.push(
            "sess-1",
            PipelineEvent::ExecutionStarted {
                session_id: "sess-1".into(),
                plan: "test".into(),
            },
        );
        store.flush().unwrap();
        let events = store.read_events("sess-1").unwrap();
        assert_eq!(events.len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn reconstruct_timeline() {
        let dir = std::env::temp_dir().join(format!("evtest-{}", rand::random::<u64>()));
        let store = EventStore::new(dir.clone());
        store.push(
            "sess-1",
            PipelineEvent::StageStarted {
                stage: "plan".into(),
            },
        );
        store.push(
            "sess-1",
            PipelineEvent::StageFinished {
                stage: "plan".into(),
                success: true,
                duration_ms: 42,
            },
        );
        store.flush().unwrap();
        let timeline = store.reconstruct_timeline("sess-1").unwrap();
        assert!(timeline.len() >= 2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn session_ids_cannot_escape_event_directory() {
        let dir = std::env::temp_dir().join(format!("evtest-{}", rand::random::<u64>()));
        let store = EventStore::new(dir.clone());
        let push_result = store.try_push(
            "../outside",
            PipelineEvent::StageStarted {
                stage: "test".into(),
            },
        );

        let flush_result = store.flush();
        let read_result = store.read_events("../outside");
        let escaped_path_exists = dir.join("outside.events.json").exists();
        let _ = std::fs::remove_dir_all(&dir);

        assert!(push_result.is_err());
        assert!(flush_result.is_ok());
        assert!(read_result.is_err());
        assert!(!escaped_path_exists);
    }
    #[test]
    fn event_paths_reject_platform_specific_escapes() {
        let dir = std::env::temp_dir().join(format!("evtest-{}", rand::random::<u64>()));
        let store = EventStore::new(dir.clone());

        for session_id in [r"..\outside", r"C:\outside", "nested/session"] {
            assert!(store.event_path(session_id).is_err());
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
    #[test]
    fn flush_does_not_duplicate_events_after_a_later_write_failure() {
        let dir = std::env::temp_dir().join(format!("evtest-{}", rand::random::<u64>()));
        let store = EventStore::new(dir.clone());
        store.push(
            "first",
            PipelineEvent::StageStarted {
                stage: "first".into(),
            },
        );
        store.push(
            "blocked",
            PipelineEvent::StageStarted {
                stage: "blocked".into(),
            },
        );
        let blocked_path = store.dir.join("blocked.events.json");
        std::fs::create_dir_all(&blocked_path).expect("create blocking directory");

        assert!(store.flush().is_err());
        std::fs::remove_dir_all(&blocked_path).expect("remove blocking directory");
        store.flush().expect("retry should flush remaining event");

        let first_count = store.read_events("first").expect("read first events").len();
        let blocked_count = store
            .read_events("blocked")
            .expect("read blocked events")
            .len();
        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(first_count, 1);
        assert_eq!(blocked_count, 1);
    }
    #[test]
    fn invalid_pushes_do_not_block_later_valid_flushes() {
        let dir = std::env::temp_dir().join(format!("evtest-{}", rand::random::<u64>()));
        let store = EventStore::new(dir.clone());
        assert!(store
            .try_push(
                "../outside",
                PipelineEvent::StageStarted {
                    stage: "invalid".into(),
                },
            )
            .is_err());
        store.push(
            "valid-session",
            PipelineEvent::StageStarted {
                stage: "valid".into(),
            },
        );
        store.flush().expect("valid event should flush");
        let events = store
            .read_events("valid-session")
            .expect("valid events should be readable");
        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(events.len(), 1);
    }
    #[test]
    fn event_paths_reject_cross_platform_collisions_and_overlong_ids() {
        let dir = std::env::temp_dir().join(format!("evtest-{}", rand::random::<u64>()));
        let store = EventStore::new(dir.clone());
        let overlong_id = "a".repeat(129);

        for session_id in ["RunA", "NUL", "COM1", "LPT9"] {
            assert!(store.event_path(session_id).is_err());
        }
        assert!(store.event_path(&overlong_id).is_err());

        let _ = std::fs::remove_dir_all(&dir);
    }
    #[test]
    fn empty_session() {
        let dir = std::env::temp_dir().join(format!("evtest-{}", rand::random::<u64>()));
        let store = EventStore::new(dir.clone());
        assert!(store.read_events("nonexistent").is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
