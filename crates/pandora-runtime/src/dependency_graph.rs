use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub subsystem_id: String,

    pub dependencies: Vec<String>,
}

pub struct DependencyGraph {
    pub nodes: HashMap<String, DependencyNode>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn register(&mut self, node: DependencyNode) {
        println!("[DEPENDENCY] registered subsystem: {}", node.subsystem_id);

        self.nodes.insert(node.subsystem_id.clone(), node);
    }

    pub fn dependencies(&self, subsystem: &str) -> Vec<String> {
        self.nodes
            .get(subsystem)
            .map(|n| n.dependencies.clone())
            .unwrap_or_default()
    }
}
