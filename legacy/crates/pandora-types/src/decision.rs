//! Execution decisions — structured record of every runtime choice.

use serde::{Deserialize, Serialize};

/// A single architectural decision made during execution.
///
/// Records what was chosen, what was rejected, and why.
/// Enables `pandora explain` to produce causal narratives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    /// Pipeline stage where this decision was made.
    pub stage: String,
    /// The alternative that was selected.
    pub chosen: String,
    /// Why it was selected over the alternatives.
    pub reason: String,
    /// Alternatives that were considered and rejected.
    pub rejected: Vec<RejectedOption>,
    /// RFC 3339 timestamp of when the decision was recorded.
    pub timestamp: String,
    /// Confidence score (0.0–1.0) from the evaluator, if applicable.
    pub confidence: f32,
    /// Evaluation score from the most recent evaluator run.
    pub evaluation_score: f32,
    /// Provider that was used for this stage.
    pub provider: String,
    /// How long this stage took, in milliseconds.
    pub duration_ms: u64,
}

/// An alternative that was considered and rejected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectedOption {
    /// Name of the rejected alternative.
    pub name: String,
    /// Why it was rejected.
    pub reason: String,
}

impl Decision {
    /// Create a new decision with the given stage, chosen alternative, and reason.
    pub fn new(
        stage: impl Into<String>,
        chosen: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            stage: stage.into(),
            chosen: chosen.into(),
            reason: reason.into(),
            rejected: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            confidence: 0.0,
            evaluation_score: 0.0,
            provider: String::new(),
            duration_ms: 0,
        }
    }

    /// Record a rejected alternative.
    #[must_use]
    pub fn reject(mut self, name: impl Into<String>, reason: impl Into<String>) -> Self {
        self.rejected.push(RejectedOption {
            name: name.into(),
            reason: reason.into(),
        });
        self
    }
}

/// Ordered collection of decisions made during an execution.
///
/// Stored in session metadata for replay and inspection.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecisionLog {
    /// The decisions, in recording order.
    pub decisions: Vec<Decision>,
}

impl DecisionLog {
    /// Create an empty decision log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a decision.
    pub fn record(&mut self, decision: Decision) {
        self.decisions.push(decision);
    }

    /// Number of decisions recorded.
    pub fn len(&self) -> usize {
        self.decisions.len()
    }

    /// True if no decisions have been recorded.
    pub fn is_empty(&self) -> bool {
        self.decisions.is_empty()
    }
}
