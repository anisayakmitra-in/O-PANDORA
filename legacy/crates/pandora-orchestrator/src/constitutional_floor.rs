//! Constitutional floor — the inviolable audit gate.
//!
//! This is NOT a harness. It is hardcoded into `pandora-orchestrator` and
//! cannot be swapped, disabled, or bypassed. Every tool call is recorded
//! here before execution, forming a tamper-evident hash chain.
//!
//! The Governance Harness (swappable SourceHarness) configures *how* strict
//! the audit interpretation is — escalation policies, approval thresholds,
//! risk scoring. But it cannot prevent this module from recording the audit
//! entry. The floor is absolute.

use sha2::{Digest, Sha256};
use std::time::SystemTime;

/// One audit record — created before every tool call.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    /// Unique execution session id.
    pub execution_id: String,
    /// Name of the gene/tool being called.
    pub tool_name: String,
    /// The tool-call id from the LLM provider.
    pub tool_call_id: String,
    /// SHA-256 of the tool input string.
    pub input_hash: String,
    /// SHA-256 of the full tool arguments JSON.
    pub arguments_hash: String,
    /// Parliament verdict applied to this call.
    pub verdict: String,
    /// Whether execution was allowed after all checks.
    pub allowed: bool,
    /// Unix timestamp (milliseconds since epoch).
    pub timestamp_ms: u128,
    /// Hash of the previous AuditEntry (None for genesis entry).
    pub previous_hash: Option<String>,
    /// SHA-256 of this entry (computed from all fields above + previous_hash).
    pub entry_hash: String,
}

/// The inviolable audit chain.
///
/// Every tool call flows through `audit_tool_call()` before the gene
/// executes. No harness, policy, or configuration can skip this gate.
pub struct ConstitutionalFloor {
    /// The immutable audit chain.
    pub entries: Vec<AuditEntry>,
    /// Whether panic-on-tamper is enabled (for tests).
    pub verify_on_access: bool,
}

impl ConstitutionalFloor {
    /// Create a new audit chain with a genesis entry.
    pub fn new(execution_id: &str) -> Self {
        let genesis = AuditEntry {
            execution_id: execution_id.to_string(),
            tool_name: "constitutional.genesis".to_string(),
            tool_call_id: "genesis".to_string(),
            input_hash: hash_str("genesis"),
            arguments_hash: hash_str("genesis"),
            verdict: "Allow".to_string(),
            allowed: true,
            timestamp_ms: now_ms(),
            previous_hash: None,
            entry_hash: String::new(),
        };

        let entry_hash = compute_entry_hash(&genesis);
        let mut genesis = genesis;
        genesis.entry_hash = entry_hash;

        Self {
            entries: vec![genesis],
            verify_on_access: false,
        }
    }

    /// Record a tool call in the audit chain before execution.
    ///
    /// This is the single mandatory gate. Every tool call MUST pass
    /// through here. Returns the sequence number of this entry.
    #[allow(clippy::too_many_arguments)]
    pub fn audit_tool_call(
        &mut self,
        execution_id: &str,
        tool_name: &str,
        tool_call_id: &str,
        input: &str,
        arguments: &str,
        verdict: &str,
        allowed: bool,
    ) -> usize {
        let previous_hash = self.entries.last().map(|e| e.entry_hash.clone());

        let entry = AuditEntry {
            execution_id: execution_id.to_string(),
            tool_name: tool_name.to_string(),
            tool_call_id: tool_call_id.to_string(),
            input_hash: hash_str(input),
            arguments_hash: hash_str(arguments),
            verdict: verdict.to_string(),
            allowed,
            timestamp_ms: now_ms(),
            previous_hash,
            entry_hash: String::new(), // filled below
        };

        let entry_hash = compute_entry_hash(&entry);
        let mut entry = entry;
        entry.entry_hash = entry_hash;

        let seq = self.entries.len();
        self.entries.push(entry);
        seq
    }

    /// Verify the entire hash chain. Returns (valid, broken_at_index).
    pub fn verify_chain(&self) -> (bool, Option<usize>) {
        for i in 1..self.entries.len() {
            let current = &self.entries[i];
            let previous = &self.entries[i - 1];

            // Check that previous_hash points to the actual previous entry
            if current.previous_hash.as_deref() != Some(&previous.entry_hash) {
                return (false, Some(i));
            }

            // Recompute this entry's hash and compare
            let recomputed = compute_entry_hash(current);
            if recomputed != current.entry_hash {
                return (false, Some(i));
            }
        }

        (true, None)
    }

    /// Number of tool calls audited (excluding genesis).
    pub fn audit_count(&self) -> usize {
        self.entries.len().saturating_sub(1)
    }
}

/// Compute SHA-256 of concatenated entry fields.
fn compute_entry_hash(entry: &AuditEntry) -> String {
    let mut hasher = Sha256::new();

    // Hash in a deterministic order
    hasher.update(b"audit:v1");
    hasher.update(entry.execution_id.as_bytes());
    hasher.update(b"\x00");
    hasher.update(entry.tool_name.as_bytes());
    hasher.update(b"\x00");
    hasher.update(entry.tool_call_id.as_bytes());
    hasher.update(b"\x00");
    hasher.update(entry.input_hash.as_bytes());
    hasher.update(b"\x00");
    hasher.update(entry.arguments_hash.as_bytes());
    hasher.update(b"\x00");
    hasher.update(entry.verdict.as_bytes());
    hasher.update(b"\x00");
    hasher.update(if entry.allowed { b"1" } else { b"0" });
    hasher.update(b"\x00");
    hasher.update(entry.timestamp_ms.to_string().as_bytes());
    hasher.update(b"\x00");

    if let Some(ref prev) = entry.previous_hash {
        hasher.update(prev.as_bytes());
    }

    hex::encode(hasher.finalize().as_slice())
}

fn hash_str(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    hex::encode(hasher.finalize().as_slice())
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_entry_has_no_previous_hash() {
        let floor = ConstitutionalFloor::new("exec-1");
        assert_eq!(floor.entries.len(), 1);
        assert!(floor.entries[0].previous_hash.is_none());
        assert!(!floor.entries[0].entry_hash.is_empty());
    }

    #[test]
    fn chain_is_valid_immediately() {
        let floor = ConstitutionalFloor::new("exec-1");
        let (valid, broken) = floor.verify_chain();
        assert!(valid);
        assert_eq!(broken, None);
    }

    #[test]
    fn audit_entries_form_chain() {
        let mut floor = ConstitutionalFloor::new("exec-1");

        floor.audit_tool_call("exec-1", "bash", "tc-1", "ls", "{}", "Allow", true);
        floor.audit_tool_call(
            "exec-1",
            "read_file",
            "tc-2",
            "README.md",
            "{}",
            "Allow",
            true,
        );
        floor.audit_tool_call("exec-1", "rm", "tc-3", "/tmp/x", "{}", "Deny", false);

        assert_eq!(floor.audit_count(), 3);
        assert_eq!(floor.entries.len(), 4); // genesis + 3

        let (valid, broken) = floor.verify_chain();
        assert!(valid, "Chain broken at {:?}", broken);
    }

    #[test]
    fn chain_detects_tampering() {
        let mut floor = ConstitutionalFloor::new("exec-1");

        floor.audit_tool_call("exec-1", "bash", "tc-1", "ls", "{}", "Allow", true);
        floor.audit_tool_call(
            "exec-1",
            "read_file",
            "tc-2",
            "README.md",
            "{}",
            "Allow",
            true,
        );

        // Tamper with the first audit entry's verdict
        floor.entries[1].verdict = "Allow".to_string(); // was "Allow", change to "Allow" no change but let's do something real
        floor.entries[1].verdict = "Deny".to_string(); // tamper!

        let (valid, broken) = floor.verify_chain();
        assert!(!valid, "Tampered chain should be detected");
        assert_eq!(broken, Some(1)); // entry 1 (first audit entry after genesis)
    }

    #[test]
    fn denied_calls_are_recorded() {
        let mut floor = ConstitutionalFloor::new("exec-1");

        floor.audit_tool_call("exec-1", "rm", "tc-1", "/etc/hosts", "{}", "Deny", false);

        let entry = &floor.entries[1];
        assert_eq!(entry.verdict, "Deny");
        assert!(!entry.allowed);
        assert!(!entry.input_hash.is_empty()); // still hashed
    }

    #[test]
    fn zero_tool_calls_is_valid() {
        let floor = ConstitutionalFloor::new("exec-1");
        assert_eq!(floor.audit_count(), 0);
        assert!(floor.verify_chain().0);
    }
}
