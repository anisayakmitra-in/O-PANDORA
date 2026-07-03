//! Pandora DAG — directed acyclic graph for execution planning.
//!
//! Phase 1A decomposition: extracted from pandora-runtime/src/dag.rs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A node in the execution DAG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagNode {
    pub node_id: String,
    pub action: String,
    pub dependencies: Vec<String>,
    pub completed: bool,
}

/// A directed acyclic graph of execution steps.
pub struct ExecutionDag {
    pub nodes: HashMap<String, DagNode>,
}

impl Default for ExecutionDag {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionDag {
    pub fn new() -> Self {
        ExecutionDag {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: DagNode) {
        self.nodes.insert(node.node_id.clone(), node);
    }

    pub fn get_ready(&self) -> Vec<&DagNode> {
        self.nodes
            .values()
            .filter(|n| !n.completed && n.dependencies.iter().all(|d| self.nodes.get(d).map_or(false, |dep| dep.completed)))
            .collect()
    }

    pub fn mark_completed(&mut self, node_id: &str) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.completed = true;
        }
    }
}
