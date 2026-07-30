//! Pending approval store — persists approval requests that require
//! human intervention before execution can proceed.
//!
//! When Parliament returns `RequireApproval`, the runtime stores a
//! `PendingApproval` and pauses. The user runs `pandora approve <id>`
//! or `pandora reject <id>`, and on the next run the orchestrator
//! checks the stored verdict before re-executing.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
        let now = Self::now_ms();

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

    /// Create an approval bound to one exact tool invocation.
    pub fn create_for_request(
        &self,
        session_id: &str,
        tool_name: &str,
        request: &str,
        who: &str,
        reason: &str,
    ) -> PendingApproval {
        let now = Self::now_ms();
        let request_hash = Self::request_hash(session_id, tool_name, request);
        let approval = PendingApproval {
            id: format!("{}-{}-{}", session_id, tool_name, &request_hash[..16]),
            session_id: session_id.to_string(),
            tool_name: tool_name.to_string(),
            who: who.to_string(),
            reason: reason.to_string(),
            created_at_ms: now,
            expires_at_ms: Some(now + 15 * 60 * 1_000),
            status: ApprovalStatus::Pending,
        };
        self.save(&approval);
        approval
    }

    /// Load an approval by id.
    pub fn get(&self, id: &str) -> Option<PendingApproval> {
        let path = self.path_for_id(id);
        let path = if path.exists() {
            path
        } else {
            self.legacy_path_for_id(id)?
        };
        if !path.exists() {
            return None;
        }
        let content = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&content).ok()
    }

    /// Check whether one exact tool invocation has a live approval.
    pub fn is_approved_for(&self, session_id: &str, tool_name: &str, request: &str) -> bool {
        let request_hash = Self::request_hash(session_id, tool_name, request);
        let id = format!("{}-{}-{}", session_id, tool_name, &request_hash[..16]);
        let Some(approval) = self.get(&id) else {
            return false;
        };
        approval.status == ApprovalStatus::Approved
            && approval
                .expires_at_ms
                .map(|expires_at| expires_at > Self::now_ms())
                .unwrap_or(true)
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

    /// List approvals in every state, ordered from newest to oldest.
    pub fn list_all(&self) -> Vec<PendingApproval> {
        let mut approvals = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.root) {
            for entry in entries.flatten() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(approval) = serde_json::from_str::<PendingApproval>(&content) {
                        approvals.push(approval);
                    }
                }
            }
        }
        approvals.sort_by(|left, right| right.created_at_ms.cmp(&left.created_at_ms));
        approvals
    }

    /// List approvals that still require a decision.
    pub fn list_pending(&self) -> Vec<PendingApproval> {
        self.list_all()
            .into_iter()
            .filter(|approval| approval.status == ApprovalStatus::Pending)
            .collect()
    }

    fn save(&self, approval: &PendingApproval) {
        let path = self.path_for_id(&approval.id);
        if let Ok(json) = serde_json::to_string_pretty(approval) {
            std::fs::write(&path, json).ok();
        }
    }

    fn now_ms() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    }

    fn request_hash(session_id: &str, tool_name: &str, request: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(session_id.as_bytes());
        hasher.update([0]);
        hasher.update(tool_name.as_bytes());
        hasher.update([0]);
        hasher.update(request.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn path_for_id(&self, id: &str) -> PathBuf {
        let digest = Sha256::digest(id.as_bytes());
        self.root
            .join(format!("approval-{}.json", hex::encode(digest)))
    }

    fn legacy_path_for_id(&self, id: &str) -> Option<PathBuf> {
        if id.is_empty()
            || id == "."
            || id == ".."
            || id.chars().any(|character| {
                character.is_control() || matches!(character, '/' | '\\' | ':' | '*')
            })
        {
            return None;
        }
        Some(self.root.join(format!("{}.json", id)))
    }
}

#[cfg(test)]
mod tests {
    use super::{ApprovalStatus, ApprovalStore};

    fn test_store() -> (ApprovalStore, std::path::PathBuf) {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock should be after epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "pandora-approval-store-{}-{nonce}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        (ApprovalStore::new(root.clone()), root)
    }

    #[test]
    fn lists_all_states_and_filters_pending() {
        let (store, root) = test_store();
        let first = store.create("session-a", "tool-a", "user", "first");
        let second = store.create("session-b", "tool-b", "user", "second");
        let third = store.create("session-c", "tool-c", "user", "third");

        store.approve(&first.id).expect("approval should succeed");
        store.reject(&second.id).expect("rejection should succeed");

        let all = store.list_all();
        assert_eq!(all.len(), 3);
        assert!(all
            .iter()
            .any(|approval| approval.status == ApprovalStatus::Approved));
        assert!(all
            .iter()
            .any(|approval| approval.status == ApprovalStatus::Rejected));
        assert!(all.iter().any(|approval| approval.id == third.id));
        assert_eq!(store.list_pending().len(), 1);

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn exact_request_approval_does_not_cover_other_arguments() {
        let (store, root) = test_store();
        let approval = store.create_for_request(
            "session-a",
            "tool-a",
            r#"{"path":"safe.txt"}"#,
            "user",
            "review",
        );
        store
            .approve(&approval.id)
            .expect("approval should succeed");

        assert!(store.is_approved_for("session-a", "tool-a", r#"{"path":"safe.txt"}"#));
        assert!(!store.is_approved_for("session-a", "tool-a", r#"{"path":"other.txt"}"#));

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn approval_ids_cannot_escape_store_directory() {
        let (store, root) = test_store();
        let approval = store.create("session/..", "tool\\escape", "user", "unsafe id");

        assert!(store.get(&approval.id).is_some());
        assert!(store
            .list_all()
            .iter()
            .any(|candidate| candidate.id == approval.id));
        assert!(!root.join("..\\escape.json").exists());

        let _ = std::fs::remove_dir_all(root);
    }
}
