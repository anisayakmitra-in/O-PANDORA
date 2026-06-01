use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub node_id: String,

    pub address: String,

    pub online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPacket {
    pub source: String,

    pub target: String,

    pub payload: String,
}

pub struct DistributedNetworkFabric {
    pub nodes: HashMap<String, NetworkNode>,
}

impl DistributedNetworkFabric {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn register_node(&mut self, node: NetworkNode) {
        println!("[NETWORK] registering {}", node.node_id);

        self.nodes.insert(node.node_id.clone(), node);
    }

    pub fn transmit(&self, packet: &NetworkPacket) -> bool {
        println!("[NETWORK] {} -> {}", packet.source, packet.target);

        match self.nodes.get(&packet.target) {
            Some(node) if node.online => {
                println!("[NETWORK] payload={}", packet.payload);

                true
            }

            _ => {
                println!("[NETWORK] transmission failed");

                false
            }
        }
    }

    pub fn online_nodes(&self) -> usize {
        self.nodes.values().filter(|node| node.online).count()
    }
}
