use crate::temporal::TemporalMemory;

use crate::category::CognitionCategory;

use std::collections::HashMap;

use crate::storage::MemoryRecord;

pub fn build_graph(memories: &[MemoryRecord]) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for memory in memories {
        graph.insert(memory.id.clone(), memory.related_memories.clone());
    }

    graph
}

pub fn multi_hop_traversal(
    graph: &HashMap<String, Vec<String>>,

    start: &str,

    depth: usize,
) -> Vec<String> {
    let mut visited = Vec::new();

    fn walk(
        graph: &HashMap<String, Vec<String>>,

        current: &str,

        depth: usize,

        visited: &mut Vec<String>,
    ) {
        if depth == 0 {
            return;
        }

        if let Some(edges) = graph.get(current) {
            for edge in edges {
                if !visited.contains(edge) {
                    visited.push(edge.clone());

                    walk(graph, edge, depth - 1, visited);
                }
            }
        }
    }

    walk(graph, start, depth, &mut visited);

    visited
}

pub fn temporal_memories(memories: &[MemoryRecord]) -> Vec<MemoryRecord> {
    let mut sorted = memories.to_vec();

    sorted.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    sorted
}

pub fn graph_index(memories: &[MemoryRecord]) -> HashMap<String, Vec<String>> {
    build_graph(memories)
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Semantic,

    Capability,

    Mutation,

    Deliberation,

    Lineage,

    Telemetry,

    Planning,

    Reasoning,

    Execution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub node_id: String,

    pub namespace: String,

    pub category: CognitionCategory,

    pub temporal: TemporalMemory,

    pub label: String,

    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEdge {
    pub edge_id: String,

    pub source: String,

    pub target: String,

    pub relationship: RelationshipType,

    pub weight: f32,
}

#[derive(Debug, Default)]
pub struct MemoryGraph {
    pub nodes: Vec<MemoryNode>,

    pub edges: Vec<MemoryEdge>,

    pub node_index: HashMap<String, usize>,

    pub adjacency_index: HashMap<String, Vec<String>>,
}

impl MemoryGraph {
    pub fn add_node(&mut self, node: MemoryNode) {
        let index = self.nodes.len();

        self.node_index.insert(node.node_id.clone(), index);

        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: MemoryEdge) {
        self.adjacency_index
            .entry(edge.source.clone())
            .or_default()
            .push(edge.edge_id.clone());

        self.adjacency_index
            .entry(edge.target.clone())
            .or_default()
            .push(edge.edge_id.clone());

        self.edges.push(edge);
    }

    pub fn neighbors(&self, node_id: &str) -> Vec<&MemoryEdge> {
        let Some(edge_ids) = self.adjacency_index.get(node_id) else {
            return Vec::new();
        };

        self.edges
            .iter()
            .filter(|edge| edge_ids.contains(&edge.edge_id))
            .collect()
    }

    pub fn namespace_nodes(&self, namespace: &str) -> Vec<&MemoryNode> {
        self.nodes
            .iter()
            .filter(|node| node.namespace == namespace)
            .collect()
    }
}
