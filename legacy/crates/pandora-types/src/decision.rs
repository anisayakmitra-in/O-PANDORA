//! Execution decisions — structured record of every runtime choice.
//!
//! Schema locked for Phase 8. Every execution produces a DecisionLog
//! entry with these mandatory fields, enabling replay, audit, and
//! future GEPA self-evolution.

use serde::{Deserialize, Serialize};

/// The outcome of a decision — success or failure with diagnostics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    /// Whether the decision led to a successful result.
    pub success: bool,
    /// Error kind if failed (e.g., "timeout", "governance", "provider_error").
    pub error_kind: Option<String>,
    /// Duration of this stage in milliseconds.
    pub duration_ms: u64,
    /// Token cost for this stage (0 if not applicable).
    pub token_cost: u64,
}

impl Outcome {
    pub fn success() -> Self {
        Self { success: true, error_kind: None, duration_ms: 0, token_cost: 0 }
    }
    pub fn failure(kind: impl Into<String>) -> Self {
        Self { success: false, error_kind: Some(kind.into()), duration_ms: 0, token_cost: 0 }
    }
}

/// A single architectural decision made during execution.
///
/// Records what was chosen, what was rejected, and why.
/// Schema locked for Phase 8 — all fields are mandatory and
/// populated by the orchestrator at each pipeline stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    /// Unique session identifier.
    pub session_id: String,
    /// Turn number within the session.
    pub turn: u32,
    /// Pipeline stage where this decision was made.
    pub stage: String,
    /// The alternative that was selected.
    pub chosen: String,
    /// Why it was selected over the alternatives.
    pub reason: String,
    /// Selected gene (if applicable).
    pub selected_gene: Option<String>,
    /// Selected harness (if applicable).
    pub selected_harness: Option<String>,
    /// Selected provider.
    pub selected_provider: Option<String>,
    /// Alternatives that were considered and rejected.
    pub rejected: Vec<RejectedOption>,
    /// RFC 3339 timestamp of when the decision was recorded.
    pub timestamp: String,
    /// Confidence score (0.0–1.0).
    pub confidence: f32,
    /// Evaluation score from the most recent evaluator run.
    pub evaluation_score: f32,
    /// Provider that was used for this stage.
    pub provider: String,
    /// Snapshot of the execution plan at this point (JSON).
    pub plan_snapshot: Option<String>,
    /// SHA-256 hash of the final output (populated on final decision).
    pub final_output_hash: Option<String>,
    /// The outcome of this decision.
    pub outcome: Outcome,
}

/// An alternative that was considered and rejected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectedOption {
    /// Name of the rejected alternative.
    pub name: String,
    /// Why it was rejected.
    pub reason: String,
    /// Confidence that this was the wrong choice (0.0–1.0).
    pub confidence: f32,
}

impl Decision {
    /// Create a new decision with required fields.
    pub fn new(
        session_id: impl Into<String>,
        turn: u32,
        stage: impl Into<String>,
        chosen: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            turn,
            stage: stage.into(),
            chosen: chosen.into(),
            reason: reason.into(),
            selected_gene: None,
            selected_harness: None,
            selected_provider: None,
            rejected: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            confidence: 0.0,
            evaluation_score: 0.0,
            provider: String::new(),
            plan_snapshot: None,
            final_output_hash: None,
            outcome: Outcome::success(),
        }
    }

    /// Record a rejected alternative with confidence.
    #[must_use]
    pub fn reject(mut self, name: impl Into<String>, reason: impl Into<String>) -> Self {
        self.rejected.push(RejectedOption {
            name: name.into(),
            reason: reason.into(),
            confidence: 0.0,
        });
        self
    }

    /// Set the selected gene for this decision.
    #[must_use]
    pub fn with_gene(mut self, gene: impl Into<String>) -> Self {
        self.selected_gene = Some(gene.into());
        self
    }

    /// Set the selected harness for this decision.
    #[must_use]
    pub fn with_harness(mut self, harness: impl Into<String>) -> Self {
        self.selected_harness = Some(harness.into());
        self
    }

    /// Set the selected provider for this decision.
    #[must_use]
    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.selected_provider = Some(provider.into());
        self
    }

    /// Set the outcome of this decision.
    #[must_use]
    pub fn with_outcome(mut self, outcome: Outcome) -> Self {
        self.outcome = outcome;
        self
    }

    /// Set the plan snapshot.
    #[must_use]
    pub fn with_plan(mut self, plan: impl Into<String>) -> Self {
        self.plan_snapshot = Some(plan.into());
        self
    }

    /// Set the provider name.
    #[must_use]
    pub fn set_provider(mut self, p: impl Into<String>) -> Self {
        self.provider = p.into();
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

    /// Get decisions filtered by stage.
    pub fn by_stage(&self, stage: &str) -> Vec<&Decision> {
        self.decisions.iter().filter(|d| d.stage == stage).collect()
    }

    /// Get failed decisions (for self-evolution analysis).
    pub fn failures(&self) -> Vec<&Decision> {
        self.decisions.iter().filter(|d| !d.outcome.success).collect()
    }
}
