use anubis_memory::category::CognitionCategory;
use anubis_memory::graph::{MemoryGraph, MemoryNode};
use anubis_memory::replay::ReplayEngine;
use anubis_memory::temporal::TemporalMemory;

fn main() {
    let mut graph = MemoryGraph::default();

    graph.add_node(MemoryNode {
        node_id: String::from("reasoning-1"),
        namespace: String::from("shadow"),
        category: CognitionCategory::Reasoning,
        temporal: TemporalMemory {
            memory_id: String::from("temporal-1"),
            timestamp: String::from("1000"),
            sequence: 1,
            recency_score: 1.0,
        },
        label: String::from("Reasoning"),
        content: String::from("Evaluate mutation"),
    });

    graph.add_node(MemoryNode {
        node_id: String::from("reasoning-2"),
        namespace: String::from("shadow"),
        category: CognitionCategory::Planning,
        temporal: TemporalMemory {
            memory_id: String::from("temporal-2"),
            timestamp: String::from("2000"),
            sequence: 2,
            recency_score: 0.9,
        },
        label: String::from("Planning"),
        content: String::from("Approve evolution"),
    });

    let replay = ReplayEngine::replay(&graph);

    println!("{:#?}", replay);
}
