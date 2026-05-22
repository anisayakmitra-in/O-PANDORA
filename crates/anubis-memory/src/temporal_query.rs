use crate::graph::{
    MemoryGraph,
    MemoryNode,
};

pub struct TemporalQueryEngine;

impl TemporalQueryEngine {

    pub fn nodes_after<'a>(

        graph:
            &'a MemoryGraph,

        timestamp:
            String,

    ) -> Vec<&'a MemoryNode> {

        graph
            .nodes
            .iter()
            .filter(
                |node| {

                    node.temporal.timestamp
                        >
                        timestamp.clone()
                }
            )
            .collect()
    }

    pub fn chronological<'a>(

        graph:
            &'a MemoryGraph,

    ) -> Vec<&'a MemoryNode> {

        let mut nodes =
            graph
                .nodes
                .iter()
                .collect::<Vec<_>>();

        nodes.sort_by_key(
            |node| {

                node.temporal
                    .timestamp
                    .clone()
            }
        );

        nodes
    }
}
