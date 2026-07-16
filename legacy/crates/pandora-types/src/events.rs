//! PipelineEvent — real-time execution events.
//!
//! Emitted during `run()` and consumed by TUI, CLI, Session Recorder,
//! Telemetry, and any other subscriber. All 17 variants cover the full
//! execution lifecycle. Publishers use the `EventSink` trait for
//! decoupled delivery — implementations can broadcast, log, stream to
//! WebSocket, or drop events entirely.

use serde::{Deserialize, Serialize};

/// Events emitted during execution pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineEvent {
    /// Execution started with the given plan specification.
    ExecutionStarted { session_id: String, plan: String },
    /// A pipeline stage started.
    StageStarted { stage: String },
    /// A pipeline stage completed.
    StageFinished {
        stage: String,
        success: bool,
        duration_ms: u64,
    },
    /// The Shadow Council selected a harness.
    HarnessSelected { harness: String, reason: String },
    /// A provider was selected.
    ProviderSelected {
        provider: String,
        model: String,
        reason: String,
    },
    /// A gene was executed.
    GeneExecuted {
        gene: String,
        duration_ms: u64,
        success: bool,
    },
    /// A control decision was made.
    DecisionMade {
        stage: String,
        chosen: String,
        reason: String,
        rejected: Vec<String>,
    },
    /// A retry was triggered.
    RetryStarted { attempt: u32, max_attempts: u32 },
    /// A retry completed.
    RetryFinished { attempt: u32, success: bool },
    /// Evaluator passed.
    EvaluationPassed { evaluator: String, goal: String },
    /// Evaluator failed with a reason.
    EvaluationFailed {
        evaluator: String,
        goal: String,
        reason: String,
    },
    /// Human approval requested for an action.
    ApprovalRequested { action: String, session_id: String },
    /// Execution completed.
    ExecutionCompleted {
        session_id: String,
        success: bool,
        duration_ms: u64,
    },
    /// Generic log message.
    Log { level: String, message: String },
}

impl PipelineEvent {
    /// Convenience constructor for [`StageStarted`].
    pub fn stage(stage: &str) -> Self {
        Self::StageStarted {
            stage: stage.into(),
        }
    }

    /// Convenience constructor for [`StageFinished`].
    pub fn stage_done(stage: &str, ok: bool) -> Self {
        Self::StageFinished {
            stage: stage.into(),
            success: ok,
            duration_ms: 0,
        }
    }

    /// Convenience constructor for [`DecisionMade`].
    pub fn decision(stage: &str, chosen: &str, reason: &str, rejected: Vec<String>) -> Self {
        Self::DecisionMade {
            stage: stage.into(),
            chosen: chosen.into(),
            reason: reason.into(),
            rejected,
        }
    }

    /// Convenience constructor for [`Log`].
    pub fn log(msg: &str) -> Self {
        Self::Log {
            level: "info".into(),
            message: msg.into(),
        }
    }
}

/// Trait for consuming [`PipelineEvent`]s.
///
/// Implementations can broadcast, log, stream to WebSocket, or ignore
/// events entirely. This decouples the runtime from its observers.
pub trait EventSink: Send + Sync {
    /// Publish an event to all subscribers of this sink.
    fn publish(&self, event: &PipelineEvent);
}

/// Sink that drops all events (for tests or `--quiet` mode).
pub struct NullSink;

impl EventSink for NullSink {
    fn publish(&self, _event: &PipelineEvent) {}
}

/// Sink that writes events to stdout as debug-formatted lines.
pub struct LoggingSink;

impl EventSink for LoggingSink {
    fn publish(&self, event: &PipelineEvent) {
        println!("[pipeline] {event:?}");
    }
}
