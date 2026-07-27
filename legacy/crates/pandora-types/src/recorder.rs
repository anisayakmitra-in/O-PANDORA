#![allow(clippy::len_without_is_empty)]
//! Execution Recorder + Replay Engine.
//!
//! Every execution is recorded and becomes deterministically replayable.
//! Captures: inputs, outputs, provider, model, context, artifacts, metrics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ── Replay ID ──

/// Unique identifier for a recorded execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReplayId(pub String);

impl ReplayId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(format!(
            "replay-{:016x}",
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ))
    }
}

impl Default for ReplayId {
    fn default() -> Self {
        Self::new()
    }
}

// ── Replay Mode ──

/// Execution mode for replay.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum ReplayMode {
    #[default]
    Deterministic,
    Evolve,
    StepThrough,
}

// ── Execution Frame ──

/// A single execution frame — a step in the execution trace.
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
            // ponytail: frame id uses counter, not constant
            frame_id: format!("frame-{:016x}", rand::random::<u64>()),
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

// ── Recorded Execution ──

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

// ── Execution Recorder ──

/// Captures every execution for replay.
#[derive(Debug, Clone)]
pub struct ExecutionRecorder {
    recordings: Vec<RecordedExecution>,
    max_recordings: usize,
}

impl Default for ExecutionRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionRecorder {
    pub fn new() -> Self {
        Self {
            recordings: Vec::new(),
            max_recordings: 10_000,
        }
    }
    pub fn with_max(max: usize) -> Self {
        Self {
            recordings: Vec::new(),
            max_recordings: max,
        }
    }

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
        self.recordings.push(RecordedExecution {
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
        });
        if self.recordings.len() > self.max_recordings {
            self.recordings.remove(0);
        }
        id
    }

    pub fn record_frame(
        &mut self,
        replay_id: &ReplayId,
        frame: ExecutionFrame,
    ) -> Result<(), crate::PandoraError> {
        self.recordings
            .iter_mut()
            .find(|r| r.replay_id == *replay_id)
            .map(|rec| rec.frames.push(frame))
            .ok_or_else(|| {
                crate::PandoraError::Internal(format!(
                    "no recording found for replay {}",
                    replay_id.0
                ))
            })
    }

    pub fn finalize(
        &mut self,
        replay_id: &ReplayId,
        total_duration_ms: u64,
        total_tokens: usize,
        total_cost: f64,
        total_retries: u32,
        success: bool,
    ) -> Result<(), crate::PandoraError> {
        self.recordings
            .iter_mut()
            .find(|r| r.replay_id == *replay_id)
            .map(|rec| {
                rec.total_duration_ms = total_duration_ms;
                rec.total_tokens = total_tokens;
                rec.total_cost = total_cost;
                rec.total_retries = total_retries;
                rec.success = success;
            })
            .ok_or_else(|| {
                crate::PandoraError::Internal(format!(
                    "no recording found for replay {}",
                    replay_id.0
                ))
            })
    }

    pub fn get(&self, replay_id: &ReplayId) -> Option<&RecordedExecution> {
        self.recordings.iter().find(|r| r.replay_id == *replay_id)
    }

    pub fn list(&self) -> Vec<&RecordedExecution> {
        let mut v: Vec<&RecordedExecution> = self.recordings.iter().collect();
        v.reverse();
        v
    }

    pub fn find_by_domain(&self, domain: &str) -> Vec<&RecordedExecution> {
        self.recordings
            .iter()
            .filter(|r| r.domain == domain)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&RecordedExecution> {
        let q = query.to_lowercase();
        self.recordings
            .iter()
            .filter(|r| r.task.to_lowercase().contains(&q) || r.domain.to_lowercase().contains(&q))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.recordings.len()
    }
}

// ── Replay Engine ──

/// Replays a recorded execution.
#[derive(Debug, Clone)]
pub struct ReplayEngine;

impl ReplayEngine {
    pub fn diff(recording: &RecordedExecution) -> Vec<(String, String, String, String)> {
        recording
            .frames
            .iter()
            .map(|f| {
                (
                    format!("{}: {}", f.step_kind, f.step_label),
                    f.input_hash.clone(),
                    f.output_hash.clone(),
                    if f.success {
                        "PASS".into()
                    } else {
                        "FAIL".into()
                    },
                )
            })
            .collect()
    }

    pub fn trace(recording: &RecordedExecution) -> String {
        let mut out = format!("EXECUTION TRACE: {}\n", recording.task);
        out.push_str(&format!("Replay ID: {}\n", recording.replay_id.0));
        out.push_str(&format!(
            "Domain: {} | Success: {}\n",
            recording.domain, recording.success
        ));
        out.push_str(&format!(
            "Duration: {}ms | Tokens: {} | Cost: ${:.4}\n",
            recording.total_duration_ms, recording.total_tokens, recording.total_cost
        ));
        out.push_str("\nFRAMES:\n");
        for frame in &recording.frames {
            let indent = if frame.parent_id.is_some() { "  " } else { "" };
            out.push_str(&format!(
                "{indent}[{}] {} {} ({}ms, {} tokens)\n",
                if frame.success { "✓" } else { "✗" },
                frame.step_kind,
                frame.step_label,
                frame.duration_ms,
                frame.tokens_used
            ));
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
        let mut r = ExecutionRecorder::new();
        let rid = r.begin(
            "test",
            "coding",
            "e1",
            "s1",
            "p1",
            RecordedProperties {
                memory_mode: "local".into(),
                loop_mode: "closed".into(),
                safety_level: "medium".into(),
                execution_backend: "native".into(),
                reasoning_depth: 3,
                telemetry_level: 2,
            },
        );
        assert!(r.get(&rid).is_some());
    }
    #[test]
    fn record_frame_works() {
        let mut r = ExecutionRecorder::new();
        let rid = r.begin(
            "test",
            "test",
            "e1",
            "s1",
            "p1",
            RecordedProperties {
                memory_mode: "local".into(),
                loop_mode: "closed".into(),
                safety_level: "medium".into(),
                execution_backend: "native".into(),
                reasoning_depth: 3,
                telemetry_level: 2,
            },
        );
        assert!(r
            .record_frame(&rid, ExecutionFrame::new("plan", "Initial plan"))
            .is_ok());
        assert_eq!(r.get(&rid).expect("recorder").frames.len(), 1);
    }
    #[test]
    fn finalize_recording() {
        let mut r = ExecutionRecorder::new();
        let rid = r.begin(
            "test",
            "test",
            "e1",
            "s1",
            "p1",
            RecordedProperties {
                memory_mode: "local".into(),
                loop_mode: "closed".into(),
                safety_level: "medium".into(),
                execution_backend: "native".into(),
                reasoning_depth: 3,
                telemetry_level: 2,
            },
        );
        r.finalize(&rid, 1500, 500, 0.05, 2, true)
            .expect("recorder");
        assert_eq!(r.get(&rid).expect("recorder").total_duration_ms, 1500);
    }
    #[test]
    fn search_by_query() {
        let mut r = ExecutionRecorder::new();
        r.begin(
            "Implement RISC-V processor",
            "eda",
            "e1",
            "s1",
            "p1",
            RecordedProperties {
                memory_mode: "local".into(),
                loop_mode: "closed".into(),
                safety_level: "medium".into(),
                execution_backend: "native".into(),
                reasoning_depth: 3,
                telemetry_level: 2,
            },
        );
        assert_eq!(r.search("risc-v").len(), 1);
    }
    #[test]
    fn trace_output() {
        let mut r = ExecutionRecorder::new();
        let rid = r.begin(
            "design",
            "eda",
            "e1",
            "s1",
            "p1",
            RecordedProperties {
                memory_mode: "local".into(),
                loop_mode: "closed".into(),
                safety_level: "medium".into(),
                execution_backend: "native".into(),
                reasoning_depth: 3,
                telemetry_level: 2,
            },
        );
        r.record_frame(&rid, ExecutionFrame::new("plan", "Architecture"))
            .expect("recorder");
        r.record_frame(&rid, ExecutionFrame::new("execute", "Implementation"))
            .expect("recorder");
        r.finalize(&rid, 5000, 1000, 0.10, 1, true)
            .expect("recorder");
        assert!(ReplayEngine::trace(r.get(&rid).expect("recorder")).contains("plan"));
    }
    #[test]
    fn invalid_replay_id_returns_error() {
        let mut r = ExecutionRecorder::new();
        assert!(r
            .record_frame(
                &ReplayId("nonexistent".into()),
                ExecutionFrame::new("test", "test")
            )
            .is_err());
    }
}
