use std::collections::HashMap;

use crate::storage::MemoryRecord;

pub fn build_graph(
    memories:
        &[MemoryRecord],
) -> HashMap<
    String,
    Vec<String>
> {

    let mut graph =
        HashMap::new();

    for memory
    in memories {

        graph.insert(
            memory.id.clone(),
            memory.related.clone(),
        );
    }

    graph
}

pub fn multi_hop_traversal(
    graph:
        &HashMap<
            String,
            Vec<String>
        >,

    start:
        &str,

    depth:
        usize,
) -> Vec<String> {

    let mut visited =
        Vec::new();

    fn walk(
        graph:
            &HashMap<
                String,
                Vec<String>
            >,

        current:
            &str,

        depth:
            usize,

        visited:
            &mut Vec<String>,
    ) {

        if depth == 0 {
            return;
        }

        if let Some(edges)
            = graph.get(current)
        {
            for edge
            in edges {

                if !visited.contains(edge)
                {
                    visited.push(
                        edge.clone()
                    );

                    walk(
                        graph,
                        edge,
                        depth - 1,
                        visited,
                    );
                }
            }
        }
    }

    walk(
        graph,
        start,
        depth,
        &mut visited,
    );

    visited
}

pub fn temporal_memories(
    memories:
        &[MemoryRecord],
) -> Vec<MemoryRecord> {

    let mut sorted =
        memories.to_vec();

    sorted.sort_by(
        |a, b| {
            a.timestamp
                .cmp(
                    &b.timestamp
                )
        }
    );

    sorted
}

pub fn graph_index(
    memories:
        &[MemoryRecord],
) -> HashMap<
    String,
    Vec<String>
> {

    build_graph(
        memories
    )
}
