//! GEPA — Gene Evolution Proposal Agent (Phase 8, read-only observer).
//!
//! Reads execution sessions and produces candidate gene/harness mutations.
//! Does NOT apply mutations automatically — that requires Parliament approval.
//!
//! This is the foundation for self-evolution: observe → propose → approve → apply.

use pandora_types::decision::{Decision, DecisionLog};
use pandora_types::session::Session;
use pandora_types::PandoraError;
use std::path::PathBuf;

/// A candidate mutation proposed by GEPA.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MutationCandidate {
    /// Unique candidate id.
    pub id: String,
    /// What kind of entity this mutation targets.
    pub target_kind: MutationTarget,
    /// The id of the target gene or harness.
    pub target_id: String,
    /// Human-readable description of the proposed change.
    pub description: String,
    /// The proposed new capability or behavior.
    pub proposal: String,
    /// Number of failures this mutation would address.
    pub failure_count: usize,
    /// Confidence in this proposal (0.0–1.0).
    pub confidence: f32,
    /// Whether this mutation has been applied.
    pub applied: bool,
    /// RFC 3339 timestamp of when this was generated.
    pub generated_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum MutationTarget {
    Gene,
    Harness,
    Provider,
}

/// Read-only observer that analyzes session history and proposes mutations.
pub struct GepaObserver {
    candidates_dir: PathBuf,
}

impl GepaObserver {
    pub fn new(root: PathBuf) -> Self {
        std::fs::create_dir_all(&root).ok();
        Self {
            candidates_dir: root,
        }
    }

    pub fn default_dir() -> PathBuf {
        std::env::var("PANDORA_HOME")
            .map(|h| PathBuf::from(h).join("mutations"))
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME")
                    .or_else(|_| std::env::var("USERPROFILE"))
                    .unwrap_or_else(|_| ".".into());
                PathBuf::from(home).join(".pandora").join("mutations")
            })
    }

    /// Analyze a session and generate mutation candidates.
    pub fn observe(&self, session: &Session) -> Vec<MutationCandidate> {
        let mut candidates = Vec::new();

        // Parse the decision log from session metadata
        let log = self.extract_decision_log(session);

        // Find failures
        let failures = log.failures();
        if failures.is_empty() {
            return candidates;
        }

        // Cluster failures by gene/harness
        let mut gene_failures: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for d in &failures {
            if let Some(ref gene) = d.selected_gene {
                *gene_failures.entry(gene.clone()).or_insert(0) += 1;
            }
        }

        // Generate candidates for frequently-failing genes
        for (gene_id, count) in &gene_failures {
            if *count >= 2 {
                let candidate = MutationCandidate {
                    id: format!(
                        "mutation-{}-{}",
                        gene_id,
                        chrono::Utc::now().timestamp_millis()
                    ),
                    target_kind: MutationTarget::Gene,
                    target_id: gene_id.clone(),
                    description: format!("Gene '{}' failed {} times", gene_id, count),
                    proposal: format!(
                        "Consider adding retry logic, better error handling, or \
                         replacing '{}' with a more robust alternative.",
                        gene_id
                    ),
                    failure_count: *count,
                    confidence: 0.3 + (0.1 * *count as f32).min(0.6),
                    applied: false,
                    generated_at: chrono::Utc::now().to_rfc3339(),
                };
                candidates.push(candidate);
            }
        }

        // Persist candidates
        for c in &candidates {
            self.save(c);
        }

        candidates
    }

    /// List all generated mutation candidates.
    pub fn list(&self) -> Vec<MutationCandidate> {
        let mut candidates = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.candidates_dir) {
            for entry in entries.flatten() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(c) = serde_json::from_str::<MutationCandidate>(&content) {
                        candidates.push(c);
                    }
                }
            }
        }
        candidates
    }

    /// Show a specific candidate.
    pub fn get(&self, id: &str) -> Option<MutationCandidate> {
        let path = self.candidates_dir.join(format!("{id}.json"));
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
    }

    /// Mark a candidate as applied.
    pub fn mark_applied(&self, id: &str) -> Result<(), PandoraError> {
        let mut c = self.get(id).ok_or_else(|| {
            PandoraError::not_found(format!("Mutation candidate not found: {id}"))
        })?;
        c.applied = true;
        self.save(&c);
        Ok(())
    }

    /// Extract decision log from session metadata.
    fn extract_decision_log(&self, session: &Session) -> DecisionLog {
        let mut log = DecisionLog::new();

        // Build decisions from the execution timeline
        for frame in &session.timeline {
            let decision = Decision::new(
                &session.id,
                0, // turn is inferred from timeline position
                &frame.step_kind,
                &frame.step_label,
                format!("{} via {}/{}", frame.step_kind, frame.provider, frame.model),
            )
            .with_provider(&frame.provider)
            .with_outcome(if frame.success {
                pandora_types::decision::Outcome::success()
            } else {
                pandora_types::decision::Outcome::failure("execution_error")
            });
            log.record(decision);
        }

        // Also parse from metadata if available
        if let Some(decision_json) = session.metadata.get("decisions") {
            if let Ok(parsed) = serde_json::from_str::<Vec<Decision>>(decision_json) {
                for d in parsed {
                    log.record(d);
                }
            }
        }

        log
    }

    fn save(&self, candidate: &MutationCandidate) {
        let path = self.candidates_dir.join(format!("{}.json", candidate.id));
        if let Ok(json) = serde_json::to_string_pretty(candidate) {
            std::fs::write(&path, json).ok();
        }
    }
}
