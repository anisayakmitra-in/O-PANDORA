use crate::graph::{MemoryGraph, MemoryNode};

pub struct TemporalQueryEngine;

impl TemporalQueryEngine {
    pub fn nodes_after(graph: &MemoryGraph, timestamp: String) -> Vec<&MemoryNode> {
        graph
            .nodes
            .iter()
            .filter(|node| node.temporal.timestamp > timestamp.clone())
            .collect()
    }

    pub fn chronological(graph: &MemoryGraph) -> Vec<&MemoryNode> {
        let mut nodes = graph.nodes.iter().collect::<Vec<_>>();

        nodes.sort_by_key(|node| node.temporal.timestamp.clone());

        nodes
    }
}
