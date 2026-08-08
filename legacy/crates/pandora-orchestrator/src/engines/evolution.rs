//! Governed RSI proposal lifecycle.

use super::mutation::{MutationEngine, MutationProposal, MutationTarget};
use pandora_types::session::Session;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvolutionStage {
    Proposed,
    AwaitingApproval,
    Approved,
    Applied,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProposal {
    pub id: String,
    pub mutation_id: String,
    pub target_kind: MutationTarget,
    pub target_id: String,
    pub rationale: String,
    pub confidence: f32,
    pub stage: EvolutionStage,
}

impl EvolutionProposal {
    fn from_mutation(proposal: MutationProposal) -> Self {
        Self {
            id: format!("rsi-{}", proposal.id),
            mutation_id: proposal.id,
            target_kind: proposal.target_kind,
            target_id: proposal.target_id,
            rationale: proposal.proposal,
            confidence: proposal.confidence,
            stage: EvolutionStage::AwaitingApproval,
        }
    }
}

pub struct EvolutionEngine<'a> {
    mutation_engine: &'a MutationEngine,
}

impl<'a> EvolutionEngine<'a> {
    pub fn new(mutation_engine: &'a MutationEngine) -> Self {
        Self { mutation_engine }
    }

    pub fn propose(&self, session: &Session) -> Vec<EvolutionProposal> {
        self.mutation_engine
            .observe(session)
            .into_iter()
            .map(EvolutionProposal::from_mutation)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposals_start_awaiting_approval() {
        let root = std::env::temp_dir().join(format!(
            "pandora-evolution-engine-{}",
            rand::random::<u64>()
        ));
        let mutation_engine = MutationEngine::new(root.clone());
        let evolution_engine = EvolutionEngine::new(&mutation_engine);
        let mut session = Session::new("session", "task");
        for index in 0..2 {
            let mut frame = pandora_types::recorder::ExecutionFrame::new("gene", "unstable-gene");
            frame.frame_id = format!("evolution-failure-{index}");
            frame.success = false;
            session.add_frame(frame);
        }

        let proposals = evolution_engine.propose(&session);

        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].stage, EvolutionStage::AwaitingApproval);
        let _ = std::fs::remove_dir_all(root);
    }
}
