use anubis_memory::graph::{MemoryEdge, MemoryGraph, MemoryNode, RelationshipType};

fn main() {
    let mut graph = MemoryGraph::default();

    graph.add_node(MemoryNode {
        node_id: String::from("reasoning-node"),

        namespace: String::from("shadow-council"),

        label: String::from("Reasoning"),

        content: String::from("Evaluate mutation"),
    });

    graph.add_node(MemoryNode {
        node_id: String::from("mutation-node"),

        namespace: String::from("gepa"),

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
