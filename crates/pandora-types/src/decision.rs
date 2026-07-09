//! Execution decisions — structured record of every runtime choice.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub stage: String,
    pub chosen: String,
    pub reason: String,
    pub rejected: Vec<RejectedOption>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectedOption {
    pub name: String,
    pub reason: String,
}

impl Decision {
    pub fn new(stage: impl Into<String>, chosen: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            stage: stage.into(),
            chosen: chosen.into(),
            reason: reason.into(),
            rejected: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    pub fn reject(mut self, name: impl Into<String>, reason: impl Into<String>) -> Self {
        self.rejected.push(RejectedOption { name: name.into(), reason: reason.into() });
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecisionLog {
    pub decisions: Vec<Decision>,
}

impl DecisionLog {
    pub fn new() -> Self { Self { decisions: Vec::new() } }
    pub fn record(&mut self, decision: Decision) { self.decisions.push(decision); }
    pub fn len(&self) -> usize { self.decisions.len() }
    pub fn is_empty(&self) -> bool { self.decisions.is_empty() }
}
