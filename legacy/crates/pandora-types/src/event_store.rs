//! Event Store — persists PipelineEvent stream for replay and analytics.
//!
//! Every execution records events to a durable log. Replay reads the log
//! and reconstructs the execution state. Analytics queries the log without
//! running the pipeline again.
//!
//! Storage: JSON-line file (~/.pandora/events/<session-id>.events.json).

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

impl EventStore {
    /// Create a new event store rooted at `base_dir/events/`.
    pub fn new(base_dir: PathBuf) -> Self {
        let dir = base_dir.join("events");
        let _ = fs::create_dir_all(&dir);
        Self { dir, buffer: Mutex::new(Vec::new()) }
    }

    /// Append an event to the in-memory buffer.
    pub fn push(&self, session_id: &str, event: PipelineEvent) {
        if let Ok(mut buf) = self.buffer.lock() {
            buf.push((session_id.to_string(), event));
        }
    }

    /// Flush all buffered events to disk.
    pub fn flush(&self) -> Result<(), String> {
        let mut buf = self.buffer.lock().map_err(|e| e.to_string())?;
        let events: Vec<(String, PipelineEvent)> = buf.drain(..).collect();
        for (session_id, event) in events {
            let path = self.dir.join(format!("{}.events.json", session_id));
            let line = serde_json::to_string(&event).map_err(|e| e.to_string())?;
            let mut file = fs::OpenOptions::new()
                .create(true).append(true).open(&path)
                .map_err(|e| format!("Cannot open event file: {e}"))?;
            use std::io::Write;
            writeln!(file, "{}", line).map_err(|e| format!("Cannot write event: {e}"))?;
        }
        Ok(())
    }

    /// Read all events for a session, in order.
    pub fn read_events(&self, session_id: &str) -> Result<Vec<PipelineEvent>, String> {
        let path = self.dir.join(format!("{}.events.json", session_id));
        let content = fs::read_to_string(&path).map_err(|e| format!("Cannot read events: {e}"))?;
        content.lines().filter(|l| !l.is_empty()).map(|l| serde_json::from_str(l).map_err(|e| format!("Parse error: {e}"))).collect()
    }

    /// Reconstruct an execution timeline from events.
    pub fn reconstruct_timeline(&self, session_id: &str) -> Result<Vec<String>, String> {
        let events = self.read_events(session_id)?;
        let mut timeline = Vec::new();
        for event in &events {
            match event {
                PipelineEvent::StageStarted { stage } => timeline.push(format!("Stage started: {stage}")),
                PipelineEvent::StageFinished { stage, success, duration_ms } => timeline.push(format!("Stage {}: {} ({}ms)", stage, if *success { "OK" } else { "FAIL" }, duration_ms)),
                PipelineEvent::HarnessSelected { harness, reason } => timeline.push(format!("Harness: {harness} ({reason})")),
                PipelineEvent::ProviderSelected { provider, model, reason } => timeline.push(format!("Provider: {provider}/{model} ({reason})")),
                PipelineEvent::GeneExecuted { gene, duration_ms, success } => timeline.push(format!("Gene: {gene} ({}ms, {})", duration_ms, if *success { "OK" } else { "FAIL" })),
                PipelineEvent::DecisionMade { stage, chosen, .. } => timeline.push(format!("Decision at {stage}: {chosen}")),
                PipelineEvent::EvaluationPassed { evaluator, .. } => timeline.push(format!("Evaluator passed: {evaluator}")),
                PipelineEvent::EvaluationFailed { evaluator, reason, .. } => timeline.push(format!("Evaluator failed: {evaluator} — {reason}")),
                _ => {}
            }
        }
        Ok(timeline)
    }

    /// Count events for a session.
    pub fn event_count(&self, session_id: &str) -> Result<usize, String> {
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
        store.push("sess-1", PipelineEvent::ExecutionStarted { session_id: "sess-1".into(), plan: "test".into() });
        store.flush().unwrap();
        let events = store.read_events("sess-1").unwrap();
        assert_eq!(events.len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn reconstruct_timeline() {
        let dir = std::env::temp_dir().join(format!("evtest-{}", rand::random::<u64>()));
        let store = EventStore::new(dir.clone());
        store.push("sess-1", PipelineEvent::StageStarted { stage: "plan".into() });
        store.push("sess-1", PipelineEvent::StageFinished { stage: "plan".into(), success: true, duration_ms: 42 });
        store.flush().unwrap();
        let timeline = store.reconstruct_timeline("sess-1").unwrap();
        assert!(timeline.len() >= 2);
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
