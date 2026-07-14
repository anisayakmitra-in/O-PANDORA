//! Pandora Distributed Registry — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeState {
    Online,

    Degraded,

    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeNode {
    pub node_id: String,

    pub address: String,

    pub capabilities: Vec<String>,

    pub state: NodeState,
}

pub struct DistributedRegistry {
    pub nodes: HashMap<String, RuntimeNode>,
}

impl Default for DistributedRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributedRegistry {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn register(&mut self, node: RuntimeNode) {
        println!("[DISTRIBUTED] node registered: {}", node.node_id);

        self.nodes.insert(node.node_id.clone(), node);
    }

    pub fn online_nodes(&self) -> usize {
        self.nodes
            .values()
            .filter(|n| matches!(n.state, NodeState::Online))
            .count()
    }
}
