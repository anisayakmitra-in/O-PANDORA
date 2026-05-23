use anubis_memory::category::CognitionCategory;

use anubis_memory::graph::{MemoryGraph, MemoryNode};

use anubis_memory::temporal::TemporalMetadata;

use anubis_memory::replay::ReplayEngine;

fn main() {
    let mut graph = MemoryGraph::default();

    graph.add_node(MemoryNode {
        node_id: String::from("reasoning-1"),

        namespace: String::from("shadow"),

        category: CognitionCategory::Reasoning,

        temporal: TemporalMetadata {
            timestamp: 1000,

            sequence: 1,
        },

        label: String::from("Reasoning"),

        content: String::from("Evaluate mutation"),
    });

    graph.add_node(MemoryNode {
        node_id: String::from("reasoning-2"),

        namespace: String::from("shadow"),

        category: CognitionCategory::Planning,

        temporal: TemporalMetadata {
            timestamp: 2000,

            sequence: 2,
        },

        label: String::from("Planning"),

        content: String::from("Approve evolution"),
    });

    let replay = ReplayEngine::replay(&graph);

    println!("{:#?}", replay);
}
