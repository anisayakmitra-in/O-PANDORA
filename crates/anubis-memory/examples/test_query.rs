use anubis_memory::graph::{MemoryEdge, MemoryGraph, MemoryNode, RelationshipType};

use anubis_memory::query::GraphQueryEngine;

fn main() {
    let mut graph = MemoryGraph::default();

    graph.add_node(MemoryNode {
        node_id: String::from("council"),

        namespace: String::from("shadow"),

        label: String::from("Council"),

        content: String::from("Evaluate mutation"),
    });

    graph.add_node(MemoryNode {
        node_id: String::from("mutation"),

        namespace: String::from("gepa"),

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
