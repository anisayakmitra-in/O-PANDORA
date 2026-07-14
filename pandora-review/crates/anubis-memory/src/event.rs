use serde::{Deserialize, Serialize};

use crate::graph::{MemoryEdge, MemoryNode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CognitionEvent {
    NodeCreated(MemoryNode),

    EdgeCreated(MemoryEdge),
}

use crate::graph::MemoryGraph;

pub struct CognitionEventStream;

impl CognitionEventStream {
    pub fn emit(graph: &mut MemoryGraph, event: CognitionEvent) {
        match event {
            CognitionEvent::NodeCreated(node) => {
                graph.add_node(node);
            }

            CognitionEvent::EdgeCreated(edge) => {
                graph.add_edge(edge);
            }
        }
    }
}
