use crate::memory_graph::{
    MemoryEdge,
    MemoryGraph,
};

pub fn connected_memories(

    graph:
        &MemoryGraph,

    node_id:
        &str,
)
    -> Vec<MemoryEdge>
{

    graph.edges
        .iter()
        .filter(
            |edge| {

                edge.from == node_id
                    || edge.to == node_id
            }
        )
        .cloned()
        .collect()
}
