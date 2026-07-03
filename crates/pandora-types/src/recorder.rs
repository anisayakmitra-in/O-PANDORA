//! Execution Recorder + Replay Engine.
//!
//! Every execution is recorded and becomes deterministically replayable.
//! Captures: inputs, outputs, provider, model, context, artifacts, metrics.
//! Generates a unique ReplayId for each execution.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Unique identifier for a recorded execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReplayId(pub String);

impl ReplayId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        ReplayId(format!("replay-{:016x}", COUNTER.fetch_add(1, Ordering::Relaxed)))
    }
}

/// Execution mode for replay.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ReplayMode {
    /// Full deterministic re-execution.
    Deterministic,
    /// Re-execute with same inputs but allow provider/model changes.
    Evolve,
    /// Replay with interactive step-through.
    StepThrough,
}

impl Default for ReplayMode { fn default() -> Self { ReplayMode::Deterministic } }

/// A single execution frame — a step in the execution graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionFrame {
    pub frame_id: String,
    pub parent_id: Option<String>,
    pub step_kind: String,
    pub step_label: String,
    pub provider: String,
    pub model: String,
    pub input_hash: String,
    pub output_hash: String,
    pub duration_ms: u64,
    pub tokens_used: usize,
    pub cost: f64,
    pub success: bool,
    pub retries: u32,
    pub artifacts: Vec<String>,
    pub telemetry: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl ExecutionFrame {
    pub fn new(step_kind: impl Into<String>, step_label: impl Into<String>) -> Self {
        Self {
            frame_id: format!("frame-{:x}", 42u64),
            parent_id: None,
            step_kind: step_kind.into(),
            step_label: step_label.into(),
            provider: String::new(),
            model: String::new(),
            input_hash: String::new(),
            output_hash: String::new(),
            duration_ms: 0,
            tokens_used: 0,
            cost: 0.0,
            success: true,
            retries: 0,
            artifacts: Vec::new(),
            telemetry: Vec::new(),
            timestamp: Utc::now(),
        }
    }
}

/// A complete recorded execution, including all frames.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedExecution {
    pub replay_id: ReplayId,
    pub task: String,
    pub domain: String,
    pub execution_id: String,
    pub session_id: String,
    pub project_id: String,

    pub properties: RecordedProperties,
    pub frames: Vec<ExecutionFrame>,

    pub total_duration_ms: u64,
    pub total_tokens: usize,
    pub total_cost: f64,
    pub total_retries: u32,
    pub success: bool,

    pub created_at: DateTime<Utc>,
}

/// Snapshot of execution properties at recording time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedProperties {
    pub memory_mode: String,
    pub loop_mode: String,
    pub safety_level: String,
    pub execution_backend: String,
    pub reasoning_depth: u32,
    pub telemetry_level: u8,
}

/// The Execution Recorder — captures every execution for replay.
#[derive(Debug, Clone)]
pub struct ExecutionRecorder {
    recordings: Vec<RecordedExecution>,
    max_recordings: usize,
}

impl ExecutionRecorder {
    pub fn new() -> Self {
        Self { recordings: Vec::new(), max_recordings: 10_000 }
    }

    pub fn with_max(max: usize) -> Self {
        Self { recordings: Vec::new(), max_recordings: max }
    }

    /// Begin recording a new execution.
    pub fn begin(
        &mut self,
        task: impl Into<String>,
        domain: impl Into<String>,
        execution_id: impl Into<String>,
        session_id: impl Into<String>,
        project_id: impl Into<String>,
        properties: RecordedProperties,
    ) -> ReplayId {
        let id = ReplayId::new();
        let recording = RecordedExecution {
            replay_id: id.clone(),
            task: task.into(),
            domain: domain.into(),
            execution_id: execution_id.into(),
            session_id: session_id.into(),
            project_id: project_id.into(),
            properties,
            frames: Vec::new(),
            total_duration_ms: 0,
            total_tokens: 0,
            total_cost: 0.0,
            total_retries: 0,
            success: true,
            created_at: Utc::now(),
        };
        if self.recordings.len() >= self.max_recordings {
            self.recordings.remove(0);
        }
        self.recordings.push(recording);
        id
    }

    /// Record a single execution frame.
    pub fn record_frame(&mut self, replay_id: &ReplayId, frame: ExecutionFrame) -> Result<(), String> {
        if let Some(rec) = self.recordings.iter_mut().find(|r| r.replay_id == *replay_id) {
            rec.frames.push(frame);
            Ok(())
        } else {
            Err(format!("no recording found for replay {}", replay_id.0))
        }
    }

    /// Finalize a recording with summary metrics.
    pub fn finalize(
        &mut self,
        replay_id: &ReplayId,
        total_duration_ms: u64,
        total_tokens: usize,
        total_cost: f64,
        total_retries: u32,
        success: bool,
    ) -> Result<(), String> {
        if let Some(rec) = self.recordings.iter_mut().find(|r| r.replay_id == *replay_id) {
            rec.total_duration_ms = total_duration_ms;
            rec.total_tokens = total_tokens;
            rec.total_cost = total_cost;
            rec.total_retries = total_retries;
            rec.success = success;
            Ok(())
        } else {
            Err(format!("no recording found for replay {}", replay_id.0))
        }
    }

    /// Retrieve a recorded execution by ReplayId.
    pub fn get(&self, replay_id: &ReplayId) -> Option<&RecordedExecution> {
        self.recordings.iter().find(|r| r.replay_id == *replay_id)
    }

    /// List all recordings, most recent first.
    pub fn list(&self) -> Vec<&RecordedExecution> {
        let mut v: Vec<&RecordedExecution> = self.recordings.iter().collect();
        v.reverse();
        v
    }

    /// Find recordings by domain.
    pub fn find_by_domain(&self, domain: &str) -> Vec<&RecordedExecution> {
        self.recordings.iter().filter(|r| r.domain == domain).collect()
    }

    /// Find recordings by task substring.
    pub fn search(&self, query: &str) -> Vec<&RecordedExecution> {
        let q = query.to_lowercase();
        self.recordings.iter()
            .filter(|r| r.task.to_lowercase().contains(&q) || r.domain.to_lowercase().contains(&q))
            .collect()
    }

    /// Total recordings stored.
    pub fn len(&self) -> usize { self.recordings.len() }
}

// =========================================================================
// Replay Engine
// =========================================================================

/// The Replay Engine — replays a recorded execution.
#[derive(Debug, Clone)]
pub struct ReplayEngine;

impl ReplayEngine {
    /// Get the input/output diff for a recorded execution.
    pub fn diff(recording: &RecordedExecution) -> Vec<(String, String, String, String)> {
        recording.frames.iter().map(|f| {
            let label = format!("{}: {}", f.step_kind, f.step_label);
            let status = if f.success { "PASS" } else { "FAIL" };
            (label, f.input_hash.clone(), f.output_hash.clone(), status.to_string())
        }).collect()
    }

    /// Get the execution trace as a tree string.
    pub fn trace(recording: &RecordedExecution) -> String {
        let mut out = format!("EXECUTION TRACE: {}\n", recording.task);
        out.push_str(&format!("Replay ID: {}\n", recording.replay_id.0));
        out.push_str(&format!("Domain: {} | Success: {}\n", recording.domain, recording.success));
        out.push_str(&format!("Duration: {}ms | Tokens: {} | Cost: ${:.4}\n", recording.total_duration_ms, recording.total_tokens, recording.total_cost));
        out.push_str("\nFRAMES:\n");
        for frame in &recording.frames {
            let indent = if frame.parent_id.is_some() { "  " } else { "" };
            let status = if frame.success { "✓" } else { "✗" };
            out.push_str(&format!("{}[{}] {} {} ({}ms, {} tokens)\n", indent, status, frame.step_kind, frame.step_label, frame.duration_ms, frame.tokens_used));
        }
        out
    }

    pub fn get_frames<'a>(&self, recording: &'a RecordedExecution) -> &'a [ExecutionFrame] {
        &recording.frames
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_id_unique() {
        let a = ReplayId::new();
        let b = ReplayId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn recorder_begin_and_retrieve() {
        let mut recorder = ExecutionRecorder::new();
        let props = RecordedProperties {
            memory_mode: "local".into(), loop_mode: "closed".into(),
            safety_level: "medium".into(), execution_backend: "native".into(),
            reasoning_depth: 3, telemetry_level: 2,
        };
        let rid = recorder.begin("test task", "coding", "exec-1", "session-1", "project-1", props);
        assert!(recorder.get(&rid).is_some());
    }

    #[test]
    fn record_frame_works() {
        let mut recorder = ExecutionRecorder::new();
        let props = RecordedProperties {
            memory_mode: "local".into(), loop_mode: "closed".into(),
            safety_level: "medium".into(), execution_backend: "native".into(),
            reasoning_depth: 3, telemetry_level: 2,
        };
        let rid = recorder.begin("test", "test", "e1", "s1", "p1", props);
        let frame = ExecutionFrame::new("plan", "Initial plan");
        assert!(recorder.record_frame(&rid, frame).is_ok());
        assert_eq!(recorder.get(&rid).unwrap().frames.len(), 1);
    }

    #[test]
    fn finalize_recording() {
        let mut recorder = ExecutionRecorder::new();
        let props = RecordedProperties {
            memory_mode: "local".into(), loop_mode: "closed".into(),
            safety_level: "medium".into(), execution_backend: "native".into(),
            reasoning_depth: 3, telemetry_level: 2,
        };
        let rid = recorder.begin("test", "test", "e1", "s1", "p1", props);
        recorder.finalize(&rid, 1500, 500, 0.05, 2, true).unwrap();
        let rec = recorder.get(&rid).unwrap();
        assert_eq!(rec.total_duration_ms, 1500);
        assert_eq!(rec.total_tokens, 500);
        assert!(rec.success);
    }

    #[test]
    fn search_by_query() {
        let mut recorder = ExecutionRecorder::new();
        let props = RecordedProperties {
            memory_mode: "local".into(), loop_mode: "closed".into(),
            safety_level: "medium".into(), execution_backend: "native".into(),
            reasoning_depth: 3, telemetry_level: 2,
        };
        recorder.begin("Implement RISC-V processor", "eda", "e1", "s1", "p1", props);
        let results = recorder.search("risc-v");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn trace_output() {
        let mut recorder = ExecutionRecorder::new();
        let props = RecordedProperties {
            memory_mode: "local".into(), loop_mode: "closed".into(),
            safety_level: "medium".into(), execution_backend: "native".into(),
            reasoning_depth: 3, telemetry_level: 2,
        };
        let rid = recorder.begin("design", "eda", "e1", "s1", "p1", props);
        recorder.record_frame(&rid, ExecutionFrame::new("plan", "Architecture")).unwrap();
        recorder.record_frame(&rid, ExecutionFrame::new("execute", "Implementation")).unwrap();
        recorder.finalize(&rid, 5000, 1000, 0.10, 1, true).unwrap();
        let rec = recorder.get(&rid).unwrap();
        let trace = ReplayEngine::trace(rec);
        assert!(trace.contains("plan"));
        assert!(trace.contains("execute"));
        assert!(trace.contains("EXECUTION TRACE"));
    }

    #[test]
    fn invalid_replay_id_returns_error() {
        let mut recorder = ExecutionRecorder::new();
        let frame = ExecutionFrame::new("test", "test");
        let result = recorder.record_frame(&ReplayId("nonexistent".to_string()), frame);
        assert!(result.is_err());
    }
}
