//! Pending approval store — persists approval requests that require
//! human intervention before execution can proceed.
//!
//! When Parliament returns `RequireApproval`, the runtime stores a
//! `PendingApproval` and pauses. The user runs `pandora approve <id>`
//! or `pandora reject <id>`, and on the next run the orchestrator
//! checks the stored verdict before re-executing.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A single pending approval request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {
    /// Unique approval id (derived from session + tool).
    pub id: String,
    /// The session this approval belongs to.
    pub session_id: String,
    /// The gene/tool that requires approval.
    pub tool_name: String,
    /// Who must approve (User, Parliament, or a specific role).
    pub who: String,
    /// Human-readable reason for the approval requirement.
    pub reason: String,
    /// When the approval request was created (unix timestamp ms).
    pub created_at_ms: u128,
    /// When the approval expires (optional).
    pub expires_at_ms: Option<u128>,
    /// Current state.
    pub status: ApprovalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

/// Disk-backed store for pending approvals.
///
/// Each approval is stored as a JSON file in `~/.pandora/approvals/<id>.json`.
/// This survives process restarts and allows the CLI to interact with
/// approvals independently of the runtime.
pub struct ApprovalStore {
    root: PathBuf,
}

impl ApprovalStore {
    /// Create or open the store at the given directory.
    pub fn new(root: PathBuf) -> Self {
        std::fs::create_dir_all(&root).ok();
        Self { root }
    }

    /// Default store location: `~/.pandora/approvals`.
    pub fn default_location() -> PathBuf {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".pandora").join("approvals")
    }

    /// Create a new pending approval.
    pub fn create(
        &self,
        session_id: &str,
        tool_name: &str,
        who: &str,
        reason: &str,
    ) -> PendingApproval {
        let id = format!("{}-{}", session_id, tool_name);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        let approval = PendingApproval {
            id: id.clone(),
            session_id: session_id.to_string(),
            tool_name: tool_name.to_string(),
            who: who.to_string(),
            reason: reason.to_string(),
            created_at_ms: now,
            expires_at_ms: None,
            status: ApprovalStatus::Pending,
        };

        self.save(&approval);
        approval
    }

    /// Load an approval by id.
    pub fn get(&self, id: &str) -> Option<PendingApproval> {
        let path = self.root.join(format!("{}.json", id));
        if !path.exists() {
            return None;
        }
        let content = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Check if a session has an approved pending request.
    pub fn is_approved(&self, session_id: &str) -> bool {
        // List all approval files and check
        if let Ok(entries) = std::fs::read_dir(&self.root) {
            for entry in entries.flatten() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(approval) = serde_json::from_str::<PendingApproval>(&content) {
                        if approval.session_id == session_id
                            && approval.status == ApprovalStatus::Approved
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Check if a session has a pending approval (not yet resolved).
    pub fn has_pending(&self, session_id: &str) -> Option<PendingApproval> {
        if let Ok(entries) = std::fs::read_dir(&self.root) {
            for entry in entries.flatten() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(approval) = serde_json::from_str::<PendingApproval>(&content) {
                        if approval.session_id == session_id
                            && approval.status == ApprovalStatus::Pending
                        {
                            return Some(approval);
                        }
                    }
                }
            }
        }
        None
    }

    /// Approve a pending request.
    pub fn approve(&self, id: &str) -> Result<PendingApproval, crate::PandoraError> {
        let mut approval = self
            .get(id)
            .ok_or_else(|| crate::PandoraError::not_found(format!("Approval not found: {id}")))?;

        if approval.status != ApprovalStatus::Pending {
            return Err(crate::PandoraError::governance(format!(
                "Approval {id} is already {:?}",
                approval.status
            )));
        }

        approval.status = ApprovalStatus::Approved;
        self.save(&approval);
        Ok(approval)
    }

    /// Reject a pending request.
    pub fn reject(&self, id: &str) -> Result<PendingApproval, crate::PandoraError> {
        let mut approval = self
            .get(id)
            .ok_or_else(|| crate::PandoraError::not_found(format!("Approval not found: {id}")))?;

        if approval.status != ApprovalStatus::Pending {
            return Err(crate::PandoraError::governance(format!(
                "Approval {id} is already {:?}",
                approval.status
            )));
        }

        approval.status = ApprovalStatus::Rejected;
        self.save(&approval);
        Ok(approval)
    }

    /// List all pending approvals.
    pub fn list_pending(&self) -> Vec<PendingApproval> {
        let mut approvals = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.root) {
            for entry in entries.flatten() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(approval) = serde_json::from_str::<PendingApproval>(&content) {
                        if approval.status == ApprovalStatus::Pending {
                            approvals.push(approval);
                        }
                    }
                }
            }
        }
        approvals
    }

    fn save(&self, approval: &PendingApproval) {
        let path = self.root.join(format!("{}.json", approval.id));
        if let Ok(json) = serde_json::to_string_pretty(approval) {
            std::fs::write(&path, json).ok();
        }
    }
}
