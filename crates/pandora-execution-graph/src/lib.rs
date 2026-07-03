//! Pandora Execution Graph — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionVertex {
    pub node_id: String,

    pub node_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConnection {
    pub from: String,

    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentExecutionGraph {
    pub graph_id: String,

    pub vertices: Vec<ExecutionVertex>,

    pub edges: Vec<ExecutionConnection>,
}

pub struct ExecutionGraphPersistence;

impl ExecutionGraphPersistence {
    pub fn persist(graph: &PersistentExecutionGraph) {
        fs::create_dir_all("execution_graphs").unwrap();

        let path = format!("execution_graphs/{}.json", graph.graph_id);

        let json = serde_json::to_string_pretty(graph).unwrap();

        fs::write(path, json).unwrap();

        println!("[GRAPH] persisted execution graph: {}", graph.graph_id);
    }
}
