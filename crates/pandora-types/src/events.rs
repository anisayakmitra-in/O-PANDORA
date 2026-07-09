//! PipelineEvent — real-time execution events.
//!
//! Emitted during run() and consumed by TUI, CLI, Session Recorder,
//! Telemetry, and any other subscriber. Uses tokio broadcast channel.

use serde::{Serialize, Deserialize};
use std::time::SystemTime;

/// Events emitted during execution pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineEvent {
    /// Execution started with given plan.
    ExecutionStarted { session_id: String, plan: String },
    /// A pipeline stage started.
    StageStarted { stage: String },
    /// A pipeline stage completed.
    StageFinished { stage: String, success: bool, duration_ms: u64 },
    /// The Shadow Council selected a harness.
    HarnessSelected { harness: String, reason: String },
    /// A provider was selected.
    ProviderSelected { provider: String, model: String, reason: String },
    /// A gene was executed.
    GeneExecuted { gene: String, duration_ms: u64, success: bool },
    /// A control decision was made.
    DecisionMade { stage: String, chosen: String, reason: String, rejected: Vec<String> },
    /// A retry was triggered.
    RetryStarted { attempt: u32, max_attempts: u32 },
    /// A retry completed.
    RetryFinished { attempt: u32, success: bool },
    /// Evaluator result.
    EvaluationPassed { evaluator: String, goal: String },
    /// Evaluator failed.
    EvaluationFailed { evaluator: String, goal: String, reason: String },
    /// Approval requested.
    ApprovalRequested { action: String, session_id: String },
    /// Execution completed.
    ExecutionCompleted { session_id: String, success: bool, duration_ms: u64 },
    /// Generic log message.
    Log { level: String, message: String },
}

impl PipelineEvent {
    pub fn stage(stage: &str) -> Self {
        PipelineEvent::StageStarted { stage: stage.into() }
    }
    pub fn stage_done(stage: &str, ok: bool) -> Self {
        PipelineEvent::StageFinished { stage: stage.into(), success: ok, duration_ms: 0 }
    }
    pub fn decision(stage: &str, chosen: &str, reason: &str, rejected: Vec<String>) -> Self {
        PipelineEvent::DecisionMade { stage: stage.into(), chosen: chosen.into(), reason: reason.into(), rejected }
    }
    pub fn log(msg: &str) -> Self {
        PipelineEvent::Log { level: "info".into(), message: msg.into() }
    }
}

