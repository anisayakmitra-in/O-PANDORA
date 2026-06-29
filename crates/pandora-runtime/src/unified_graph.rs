use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionNode {
    pub node_id: String,

    pub node_type: String,

    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEdge {
    pub source: String,

    pub target: String,

    pub relationship: String,
}

pub struct UnifiedExecutionGraph {
    pub nodes: HashMap<String, ExecutionNode>,

    pub edges: Vec<ExecutionEdge>,
}

impl Default for UnifiedExecutionGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl UnifiedExecutionGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),

            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: ExecutionNode) {
        println!("[GRAPH] node added: {}", node.node_id);

        self.nodes.insert(node.node_id.clone(), node);
    }

    pub fn add_edge(&mut self, edge: ExecutionEdge) {
        println!("[GRAPH] edge {} -> {}", edge.source, edge.target);

        self.edges.push(edge);
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}
