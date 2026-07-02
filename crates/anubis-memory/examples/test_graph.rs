use anubis_memory::category::CognitionCategory;
use anubis_memory::graph::{MemoryEdge, MemoryGraph, MemoryNode, RelationshipType};
use anubis_memory::temporal::TemporalMemory;

fn main() {
    let mut graph = MemoryGraph::default();

    graph.add_node(MemoryNode {
        node_id: String::from("reasoning-node"),
        namespace: String::from("shadow-council"),
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
        node_id: String::from("mutation-node"),
        namespace: String::from("gepa"),
        category: CognitionCategory::Mutation,
        temporal: TemporalMemory {
            memory_id: String::from("temporal-2"),
            timestamp: String::from("2000"),
            sequence: 2,
            recency_score: 0.9,
        },
        label: String::from("Mutation"),
        content: String::from("Prompt optimization"),
    });

    graph.add_edge(MemoryEdge {
        edge_id: String::from("edge-001"),
        source: String::from("reasoning-node"),
        target: String::from("mutation-node"),
        relationship: RelationshipType::Deliberation,
        weight: 0.9,
    });

    println!("{:#?}", graph.neighbors("reasoning-node"));
}
