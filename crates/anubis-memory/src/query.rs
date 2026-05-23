use crate::graph::{MemoryEdge, MemoryGraph, RelationshipType};

pub struct GraphQueryEngine;

impl GraphQueryEngine {
    pub fn edges_by_relationship<'a>(
        graph: &'a MemoryGraph,

        relationship: RelationshipType,
    ) -> Vec<&'a MemoryEdge> {
        graph
            .edges
            .iter()
            .filter(|edge| {
                std::mem::discriminant(&edge.relationship) == std::mem::discriminant(&relationship)
            })
            .collect()
    }

    pub fn lineage_neighbors<'a>(graph: &'a MemoryGraph, node_id: &str) -> Vec<&'a MemoryEdge> {
        graph
            .neighbors(node_id)
            .into_iter()
            .filter(|edge| {
                matches!(
                    edge.relationship,
                    RelationshipType::Lineage | RelationshipType::Mutation
                )
            })
            .collect()
    }

    pub fn deliberation_chain<'a>(graph: &'a MemoryGraph, node_id: &str) -> Vec<&'a MemoryEdge> {
        graph
            .neighbors(node_id)
            .into_iter()
            .filter(|edge| matches!(edge.relationship, RelationshipType::Deliberation))
            .collect()
    }

    pub fn semantic_neighbors<'a>(graph: &'a MemoryGraph, node_id: &str) -> Vec<&'a MemoryEdge> {
        graph
            .neighbors(node_id)
            .into_iter()
            .filter(|edge| matches!(edge.relationship, RelationshipType::Semantic))
            .collect()
    }
}
