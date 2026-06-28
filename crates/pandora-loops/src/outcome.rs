use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The result of running a Loop.
///
///  is what the runtime consumes after a
/// loop completes. It carries:
/// - the loop's name and a unique
/// - a status (Completed, Failed, Escalated, Skipped)
/// - a list of artifacts the loop produced
/// - a free-form notes map
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopOutcome {
    pub outcome_id: String,
    pub loop_name: String,
    pub status: LoopStatus,
    pub artifacts: Vec<OutcomeArtifact>,
    pub notes: BTreeMap<String, String>,
}

impl LoopOutcome {
    pub fn completed(loop_name: impl Into<String>) -> Self {
        LoopOutcome {
            outcome_id: generate_outcome_id(),
            loop_name: loop_name.into(),
            status: LoopStatus::Completed,
            artifacts: Vec::new(),
            notes: BTreeMap::new(),
        }
    }

    pub fn failed(loop_name: impl Into<String>, reason: impl Into<String>) -> Self {
        let mut outcome = LoopOutcome::completed(loop_name);
        outcome.status = LoopStatus::Failed;
        outcome.notes.insert("reason".to_string(), reason.into());
        outcome
    }

    pub fn with_artifact(mut self, artifact: OutcomeArtifact) -> Self {
        self.artifacts.push(artifact);
        self
    }

    pub fn with_note(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.notes.insert(key.into(), value.into());
        self
    }
}

/// Loop execution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopStatus {
    Completed,
    Failed,
    Escalated,
    Skipped,
}

impl LoopStatus {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            LoopStatus::Completed | LoopStatus::Failed | LoopStatus::Skipped
        )
    }
}

/// An artifact produced by a Loop. Artifacts are
/// structured data the runtime may pass downstream
/// or persist to ANUBIS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutcomeArtifact {
    pub kind: String,
    pub content: String,
}

impl OutcomeArtifact {
    pub fn new(kind: impl Into<String>, content: impl Into<String>) -> Self {
        OutcomeArtifact {
            kind: kind.into(),
            content: content.into(),
        }
    }
}

fn generate_outcome_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("outcome-{:016x}", n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completed_outcome_has_terminal_status() {
        let o = LoopOutcome::completed("planning");
        assert_eq!(o.status, LoopStatus::Completed);
        assert!(o.status.is_terminal());
    }

    #[test]
    fn failed_outcome_records_reason() {
        let o = LoopOutcome::failed("planning", "no plan found");
        assert_eq!(o.status, LoopStatus::Failed);
        assert_eq!(o.notes.get("reason"), Some(&"no plan found".to_string()));
    }

    #[test]
    fn artifacts_and_notes_compose() {
        let o = LoopOutcome::completed("benchmark")
            .with_artifact(OutcomeArtifact::new("score", "0.95"))
            .with_note("winner", "candidate_001");
        assert_eq!(o.artifacts.len(), 1);
        assert_eq!(o.notes.get("winner"), Some(&"candidate_001".to_string()));
    }

    #[test]
    fn outcome_id_is_unique() {
        let a = LoopOutcome::completed("a");
        let b = LoopOutcome::completed("a");
        assert_ne!(a.outcome_id, b.outcome_id);
    }

    #[test]
    fn outcome_serializes() {
        let o = LoopOutcome::completed("planning");
        let s = serde_json::to_string(&o).unwrap();
        let o2: LoopOutcome = serde_json::from_str(&s).unwrap();
        assert_eq!(o, o2);
    }
}
