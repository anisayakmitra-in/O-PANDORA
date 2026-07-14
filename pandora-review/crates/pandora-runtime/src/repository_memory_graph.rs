use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: String,

    pub content: String,

    pub links: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryMemoryGraph {
    pub nodes: HashMap<String, MemoryNode>,
}

pub struct RepositoryMemoryGraphEngine;

impl RepositoryMemoryGraphEngine {
    pub fn build(memories: &[MemoryNode]) -> RepositoryMemoryGraph {
        let mut graph = HashMap::new();

        for memory in memories {
            println!(
                "[MEMORY-GRAPH] node={} links={}",
                memory.id,
                memory.links.len()
            );

            graph.insert(memory.id.clone(), memory.clone());
        }

        RepositoryMemoryGraph { nodes: graph }
    }

    pub fn related(graph: &RepositoryMemoryGraph, node_id: &str) -> Vec<MemoryNode> {
        let mut related = Vec::new();

        if let Some(node) = graph.nodes.get(node_id) {
            for link in &node.links {
                if let Some(connected) = graph.nodes.get(link) {
                    related.push(connected.clone());
                }
            }
        }

        related
    }
}
