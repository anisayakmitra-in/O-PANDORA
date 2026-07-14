use serde::{
    Serialize,
    Deserialize,
};

use uuid::Uuid;

use crate::evolution::promotion::{
    EvolutionCandidate,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct LineageNode {

    pub candidate:
        EvolutionCandidate,

    pub children:
        Vec<Uuid>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct EvolutionLineage {

    pub root_candidate:
        Uuid,

    pub nodes:
        Vec<LineageNode>,
}

pub struct LineageManager;

impl LineageManager {

    pub fn add_candidate(

        lineage:
            &mut EvolutionLineage,

        candidate:
            EvolutionCandidate,

    ) {

        let candidate_id =
            candidate.candidate_id;

        let parent =
            candidate.parent_candidate;

        lineage.nodes.push(
            LineageNode {

                candidate,

                children:
                    Vec::new(),
            }
        );

        if let Some(parent_id) =
            parent
        {

            if let Some(node) =

                lineage
                    .nodes
                    .iter_mut()
                    .find(
                        |node| {

                            node.candidate
                                .candidate_id

                                ==

                            parent_id
                        }
                    )
            {

                node.children
                    .push(
                        candidate_id
                    );
            }
        }
    }

    pub fn descendants(

        lineage:
            &EvolutionLineage,

        candidate_id:
            Uuid,

    ) -> Vec<Uuid> {

        lineage
            .nodes
            .iter()
            .find(
                |node| {

                    node.candidate
                        .candidate_id

                        ==

                    candidate_id
                }
            )
            .map(
                |node| {
                    node.children.clone()
                }
            )
            .unwrap_or_default()
    }
}
