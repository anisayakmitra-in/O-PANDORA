use anubis_memory::category::CognitionCategory;
use anubis_memory::event::{CognitionEvent, CognitionEventStream};
use anubis_memory::graph::{MemoryEdge, MemoryGraph, MemoryNode, RelationshipType};
use anubis_memory::temporal::TemporalMemory;

fn main() {
    let mut graph = MemoryGraph::default();

    CognitionEventStream::emit(
        &mut graph,
        CognitionEvent::NodeCreated(MemoryNode {
            node_id: String::from("reasoning"),
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
        }),
    );

    CognitionEventStream::emit(
        &mut graph,
        CognitionEvent::NodeCreated(MemoryNode {
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
        }),
    );

    CognitionEventStream::emit(
        &mut graph,
        CognitionEvent::EdgeCreated(MemoryEdge {
            edge_id: String::from("edge-1"),
            source: String::from("reasoning"),
            target: String::from("mutation"),
            relationship: RelationshipType::Deliberation,
            weight: 0.95,
        }),
    );

    println!("{:#?}", graph.neighbors("reasoning"));
}
