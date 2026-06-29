use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagNode {
    pub node_id: String,

    pub action: String,

    pub dependencies: Vec<String>,

    pub completed: bool,
}

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
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: DagNode) {
        println!("[DAG] node added: {}", node.node_id);

        self.nodes.insert(node.node_id.clone(), node);
    }

    pub fn executable(&self) -> Vec<&DagNode> {
        self.nodes
            .values()
            .filter(|node| {
                if node.completed {
                    return false;
                }

                node.dependencies
                    .iter()
                    .all(|dep| self.nodes.get(dep).map(|n| n.completed).unwrap_or(false))
            })
            .collect()
    }

    pub fn execute(&mut self) {
        loop {
            let ready = self
                .executable()
                .iter()
                .map(|n| n.node_id.clone())
                .collect::<Vec<_>>();

            if ready.is_empty() {
                break;
            }

            for node_id in ready {
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    println!("[DAG] executing {} -> {}", node.node_id, node.action);

                    node.completed = true;
                }
            }
        }
    }
}
