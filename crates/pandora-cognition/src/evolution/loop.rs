use anyhow::Result;

use crate::dataset::store::DatasetStore;

use crate::evolution::MutationProposal;

use crate::evolution::engine::MutationEngine;

use crate::evolution::pareto::{
    ParetoCandidate,
    pareto_frontier,
};

use crate::optimizer::Optimizer;

pub struct EvolutionLoop<O, M>
where
    O: Optimizer,
    M: MutationEngine,
{

    optimizer:
        O,

    mutation_engine:
        M,

    dataset_store:
        DatasetStore,
}

impl<O, M> EvolutionLoop<O, M>

where

    O:
        Optimizer,

    M:
        MutationEngine,
{

    pub fn new(

        optimizer:
            O,

        mutation_engine:
            M,

        dataset_store:
            DatasetStore,

    ) -> Self {

        Self {

            optimizer,

            mutation_engine,

            dataset_store,
        }
    }

    pub async fn evolve(
        &self,
    ) -> Result<
        Vec<MutationProposal>
    > {

        let dataset =
            self.dataset_store
                .all()
                .await;

        let optimization =
            self.optimizer
                .optimize(
                    &dataset
                )
                .await?;

        let mutations =
            self.mutation_engine
                .generate_mutations(
                    &dataset
                )
                .await?;

        let candidates =
            mutations
                .iter()
                .enumerate()
                .map(
                    |(idx, _)| {

                        ParetoCandidate {

                            candidate_id:
                                format!(
                                    "candidate-{}",
                                    idx
                                ),

                            quality_score:
                                0.8,

                            safety_score:
                                0.9,

                            latency_score:
                                0.7,

                            efficiency_score:
                                0.75,

                            multilingual_score:
                                0.8,
                        }
                    }
                )
                .collect::<Vec<_>>();

        let frontier =
            pareto_frontier(
                &candidates
            );

        let selected =
            mutations
                .into_iter()
                .enumerate()
                .filter(
                    |(idx, _)| {

                        frontier
                            .iter()
                            .any(
                                |candidate| {

                                    candidate
                                        .candidate_id

                                        ==

                                    format!(
                                        "candidate-{}",
                                        idx
                                    )
                                }
                            )
                    }
                )
                .map(
                    |(_, mutation)| {
                        mutation
                    }
                )
                .collect();

        println!(
            "Optimization reasoning: {}",
            optimization.reasoning
        );

        Ok(selected)
    }
}
