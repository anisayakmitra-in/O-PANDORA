use serde::{
    Serialize,
    Deserialize,
};

use uuid::Uuid;

use crate::evolution::MutationProposal;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub enum PromotionStage {

    Experimental,

    SandboxValidated,

    GovernanceApproved,

    Production,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct EvolutionCandidate {

    pub candidate_id:
        Uuid,

    pub parent_candidate:
        Option<Uuid>,

    pub mutation:
        MutationProposal,

    pub stage:
        PromotionStage,

    pub rollback_supported:
        bool,

    pub lineage_depth:
        u32,
}

pub struct PromotionManager;

impl PromotionManager {

    pub fn promote(

        mut candidate:
            EvolutionCandidate,

    ) -> EvolutionCandidate {

        candidate.stage =

            match candidate.stage {

                PromotionStage
                    ::Experimental => {

                    PromotionStage
                        ::SandboxValidated
                }

                PromotionStage
                    ::SandboxValidated => {

                    PromotionStage
                        ::GovernanceApproved
                }

                PromotionStage
                    ::GovernanceApproved => {

                    PromotionStage
                        ::Production
                }

                PromotionStage
                    ::Production => {

                    PromotionStage
                        ::Production
                }
            };

        candidate
    }

    pub fn rollback(

        candidate:
            &EvolutionCandidate,

    ) -> Option<Uuid> {

        if candidate
            .rollback_supported
        {

            candidate
                .parent_candidate

        } else {

            None
        }
    }
}
