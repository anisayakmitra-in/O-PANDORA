use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct MemoryNode {

    pub node_id:
        String,

    pub memory_id:
        String,

    pub node_type:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct MemoryEdge {

    pub edge_id:
        String,

    pub from:
        String,

    pub to:
        String,

    pub relationship:
        String,

    pub weight:
        f32,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct MemoryGraph {

    pub graph_id:
        String,

    pub nodes:
        Vec<MemoryNode>,

    pub edges:
        Vec<MemoryEdge>,
}
