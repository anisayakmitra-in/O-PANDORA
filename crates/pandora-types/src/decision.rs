//! Execution decisions — structured record of every runtime choice.
//!
//! The ExecutionController records WHY each decision was made, what
//! alternatives existed, and why they were rejected.
//!
//! This enables introspection: `pandora inspect <id>` prints the
//! decision log without reverse-engineering anything.

use serde::{Deserialize, Serialize};

/// A single runtime decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    /// Identifies the pipeline stage (e.g. "provider-selection", "harness-dispatch").
    pub stage: String,
    /// What was chosen.
    pub chosen: String,
    /// Why it was chosen.
    pub reason: String,
    /// Alternatives considered but rejected.
    pub rejected: Vec<RejectedOption>,
    /// Timing.
    pub timestamp: String,
}

/// An option that was considered and rejected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectedOption {
    pub name: String,
    pub reason: String,
}

impl Decision {
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
        }
    }

    pub fn reject(mut self, name: impl Into<String>, reason: impl Into<String>) -> Self {
        self.rejected.push(RejectedOption {
            name: name.into(),
            reason: reason.into(),
        });
        self
    }
}

/// Ordered log of decisions made during execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecisionLog {
    pub decisions: Vec<Decision>,
}

impl DecisionLog {
    pub fn new() -> Self {
        Self {
            decisions: Vec::new(),
        }
    }

    pub fn record(&mut self, decision: Decision) {
        self.decisions.push(decision);
    }

    pub fn len(&self) -> usize { self.decisions.len() }
    pub fn is_empty(&self) -> bool { self.decisions.is_empty() }
        self.decisions.len()
    }
}
