//! GEPA-backed mutation proposal engine.
//!
//! The engine observes completed sessions and writes mutation proposals. It
//! never applies a proposal; activation remains subject to governance.

use pandora_types::decision::{Decision, DecisionLog};
use pandora_types::session::Session;
use pandora_types::PandoraError;
use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MutationProposal {
    pub id: String,
    pub target_kind: MutationTarget,
    pub target_id: String,
    pub description: String,
    pub proposal: String,
    pub failure_count: usize,
    pub confidence: f32,
    pub applied: bool,
    pub generated_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum MutationTarget {
    Gene,
    Harness,
    Provider,
}

pub struct MutationEngine {
    proposals_dir: PathBuf,
}

impl MutationEngine {
    pub fn new(root: PathBuf) -> Self {
        std::fs::create_dir_all(&root).ok();
        Self {
            proposals_dir: root,
        }
    }

    pub fn default_dir() -> PathBuf {
        std::env::var("PANDORA_HOME")
            .map(|home| PathBuf::from(home).join("mutations"))
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME")
                    .or_else(|_| std::env::var("USERPROFILE"))
                    .unwrap_or_else(|_| ".".into());
                PathBuf::from(home).join(".pandora").join("mutations")
            })
    }

    pub fn observe(&self, session: &Session) -> Vec<MutationProposal> {
        let decision_log = self.extract_decision_log(session);
        let failures = decision_log.failures();
        if failures.is_empty() {
            return Vec::new();
        }

        let mut gene_failures = std::collections::HashMap::<String, usize>::new();
        for decision in failures {
            if let Some(gene) = &decision.selected_gene {
                *gene_failures.entry(gene.clone()).or_insert(0) += 1;
            }
        }

        let proposals = gene_failures
            .into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(gene_id, count)| MutationProposal {
                id: format!(
                    "mutation-{}-{}",
                    safe_id_component(&gene_id),
                    chrono::Utc::now().timestamp_millis()
                ),
                target_kind: MutationTarget::Gene,
                target_id: gene_id.clone(),
                description: format!("Gene '{gene_id}' failed {count} times"),
                proposal: format!(
                    "Consider adding retry logic, better error handling, or replacing '{gene_id}' with a more robust alternative."
                ),
                failure_count: count,
                confidence: 0.3 + (0.1 * count as f32).min(0.6),
                applied: false,
                generated_at: chrono::Utc::now().to_rfc3339(),
            })
            .collect::<Vec<_>>();

        for proposal in &proposals {
            self.save(proposal);
        }
        proposals
    }

    pub fn list(&self) -> Vec<MutationProposal> {
        let mut proposals = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.proposals_dir) {
            for entry in entries.flatten() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    if let Ok(proposal) = serde_json::from_str::<MutationProposal>(&content) {
                        proposals.push(proposal);
                    }
                }
            }
        }
        proposals
    }

    pub fn get(&self, id: &str) -> Option<MutationProposal> {
        if !is_safe_proposal_id(id) {
            return None;
        }
        let path = self.proposals_dir.join(format!("{id}.json"));
        std::fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
    }

    pub fn mark_applied(&self, id: &str) -> Result<(), PandoraError> {
        let mut proposal = self.get(id).ok_or_else(|| {
            PandoraError::not_found(format!("Mutation candidate not found: {id}"))
        })?;
        proposal.applied = true;
        self.save(&proposal);
        Ok(())
    }

    fn extract_decision_log(&self, session: &Session) -> DecisionLog {
        let mut log = DecisionLog::new();
        for (turn, frame) in session.timeline.iter().enumerate() {
            let mut decision = Decision::new(
                &session.id,
                turn as u32,
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
            if frame.step_kind == "gene" {
                decision = decision.with_gene(&frame.step_label);
            }
            log.record(decision);
        }

        if let Some(decision_json) = session.metadata.get("decisions") {
            if let Ok(decisions) = serde_json::from_str::<Vec<Decision>>(decision_json) {
                for decision in decisions {
                    log.record(decision);
                }
            }
        }
        log
    }

    fn save(&self, proposal: &MutationProposal) {
        if !is_safe_proposal_id(&proposal.id) {
            return;
        }
        let path = self.proposals_dir.join(format!("{}.json", proposal.id));
        if let Ok(json) = serde_json::to_string_pretty(proposal) {
            std::fs::write(path, json).ok();
        }
    }
}

fn safe_id_component(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .take(80)
        .collect()
}

fn is_safe_proposal_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 160
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposal_ids_cannot_escape_storage_directory() {
        assert_eq!(safe_id_component("../secret"), "___secret");
        assert!(!is_safe_proposal_id("../secret"));
        assert!(is_safe_proposal_id("mutation-gene-1"));
    }

    #[test]
    fn repeated_gene_failures_create_mutation_proposal() {
        let root =
            std::env::temp_dir().join(format!("pandora-mutation-engine-{}", rand::random::<u64>()));
        let mut session = Session::new("session", "task");
        for index in 0..2 {
            let mut frame = pandora_types::recorder::ExecutionFrame::new("gene", "unstable-gene");
            frame.frame_id = format!("failed-gene-{index}");
            frame.success = false;
            session.add_frame(frame);
        }

        let engine = MutationEngine::new(root.clone());
        let proposals = engine.observe(&session);

        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].target_id, "unstable-gene");
        assert_eq!(proposals[0].failure_count, 2);
        let _ = std::fs::remove_dir_all(root);
    }
}
