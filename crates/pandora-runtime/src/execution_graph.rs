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
pub struct ExecutionGraph {

    pub graph_id:
        String,

    pub root_task_id:
        String,

    pub nodes:
        Vec<ExecutionNode>,

    pub edges:
        Vec<ExecutionEdge>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct ExecutionNode {

    pub node_id:
        String,

    pub gene_id:
        String,

    pub harness_id:
        String,

    pub status:
        ExecutionStatus,

    pub capability:
        String,

    pub timestamp:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct ExecutionEdge {

    pub from:
        String,

    pub to:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub enum ExecutionStatus {

    Pending,

    Running,

    Completed,

    Failed,

    Governed,

    Rejected,
}
