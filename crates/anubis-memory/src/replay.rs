use crate::graph::{MemoryGraph, MemoryNode};

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay<'a>(graph: &'a MemoryGraph) -> Vec<&'a MemoryNode> {
        let mut timeline = graph.nodes.iter().collect::<Vec<_>>();

        timeline.sort_by_key(|node| node.temporal.timestamp.clone());

        timeline
    }
}

impl ReplayEngine {
    pub fn replay_window<'a>(
        graph: &'a MemoryGraph,

        start: String,

        end: String,
    ) -> Vec<&'a MemoryNode> {
        let mut timeline = graph
            .nodes
            .iter()
            .filter(|node| {
                node.temporal.timestamp >= start.clone() && node.temporal.timestamp <= end.clone()
            })
            .collect::<Vec<_>>();

        timeline.sort_by_key(|node| node.temporal.timestamp.clone());

        timeline
    }
}
