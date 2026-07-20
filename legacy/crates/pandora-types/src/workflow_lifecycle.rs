//! Workflow Lifecycle — canonical workflow state machine.
//!
//! Every workflow passes through: Initialize → Plan → Execute → Verify →
//! Recover → Complete (or Abort). Middleware and event emission at each
//! state transition. This complements the existing WorkflowEngine which
//! focuses on graph-based execution planning.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleState {
    Initialize,
    Plan,
    Execute,
    Verify,
    Recover,
    Complete,
    Abort,
}

impl LifecycleState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::Plan => "plan",
            Self::Execute => "execute",
            Self::Verify => "verify",
            Self::Recover => "recover",
            Self::Complete => "complete",
            Self::Abort => "abort",
        }
    }
    pub fn next_states(&self) -> &[LifecycleState] {
        match self {
            Self::Initialize => &[Self::Plan],
            Self::Plan => &[Self::Execute, Self::Abort],
            Self::Execute => &[Self::Verify, Self::Recover, Self::Abort],
            Self::Verify => &[Self::Complete, Self::Recover, Self::Execute, Self::Abort],
            Self::Recover => &[Self::Execute, Self::Abort],
            Self::Complete | Self::Abort => &[],
        }
    }
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Complete | Self::Abort)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleMiddleware {
    pub name: String,
    pub before: Vec<LifecycleState>,
    pub after: Vec<LifecycleState>,
    pub blocking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleStep {
    pub name: String,
    pub state: LifecycleState,
    pub started_at: Option<SystemTime>,
    pub completed_at: Option<SystemTime>,
    pub error: Option<String>,
    pub retries: u32,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lifecycle {
    pub id: String,
    pub name: String,
    pub state: LifecycleState,
    pub steps: Vec<LifecycleStep>,
    pub middleware: Vec<LifecycleMiddleware>,
    pub created_at: SystemTime,
    pub completed_at: Option<SystemTime>,
}

impl Lifecycle {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            state: LifecycleState::Initialize,
            steps: vec![],
            middleware: vec![],
            created_at: SystemTime::now(),
            completed_at: None,
        }
    }
    pub fn transition(&mut self, next: LifecycleState) -> Result<(), String> {
        if !self.state.next_states().contains(&next) {
            return Err(format!(
                "Invalid transition: {} → {}",
                self.state.label(),
                next.label()
            ));
        }
        self.state = next;
        if next.is_terminal() {
            self.completed_at = Some(SystemTime::now());
        }
        Ok(())
    }
    pub fn step(&mut self, name: &str, max_retries: u32) {
        self.steps.push(LifecycleStep {
            name: name.into(),
            state: self.state,
            started_at: Some(SystemTime::now()),
            completed_at: None,
            error: None,
            retries: 0,
            max_retries,
        });
    }
    pub fn can_retry(&self) -> bool {
        self.steps.last().is_some_and(|s| s.retries < s.max_retries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn happy_path() {
        let mut l = Lifecycle::new("l1", "test");
        assert!(l.transition(LifecycleState::Plan).is_ok());
        assert!(l.transition(LifecycleState::Execute).is_ok());
        assert!(l.transition(LifecycleState::Verify).is_ok());
        assert!(l.transition(LifecycleState::Complete).is_ok());
        assert!(l.state.is_terminal());
    }
    #[test]
    fn invalid_rejected() {
        let mut l = Lifecycle::new("l2", "test");
        assert!(l.transition(LifecycleState::Complete).is_err());
    }
}
