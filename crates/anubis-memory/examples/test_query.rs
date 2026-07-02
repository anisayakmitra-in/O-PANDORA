use anubis_memory::category::CognitionCategory;
use anubis_memory::graph::{MemoryEdge, MemoryGraph, MemoryNode, RelationshipType};
use anubis_memory::query::GraphQueryEngine;
use anubis_memory::temporal::TemporalMemory;

fn main() {
    let mut graph = MemoryGraph::default();

    graph.add_node(MemoryNode {
        node_id: String::from("council"),
        namespace: String::from("shadow"),
        category: CognitionCategory::Reasoning,
        temporal: TemporalMemory {
            memory_id: String::from("temporal-1"),
            timestamp: String::from("1000"),
            sequence: 1,
            recency_score: 1.0,
        },
        label: String::from("Council"),
        content: String::from("Evaluate mutation"),
    });

    graph.add_node(MemoryNode {
        node_id: String::from("mutation"),
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
        edge_id: String::from("edge-1"),
        source: String::from("council"),
        target: String::from("mutation"),
        relationship: RelationshipType::Deliberation,
        weight: 1.0,
    });

    let results = GraphQueryEngine::deliberation_chain(&graph, "council");

    println!("{:#?}", results);
}
