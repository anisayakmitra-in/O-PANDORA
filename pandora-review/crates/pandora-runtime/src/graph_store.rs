use std::fs;

use crate::execution_graph::PersistentExecutionGraph;

pub fn persist_graph(graph: &PersistentExecutionGraph) {
    fs::create_dir_all("graphs").unwrap();

    let path = format!("graphs/{}.json", graph.graph_id);

    let json = serde_json::to_string_pretty(graph).unwrap();

    fs::write(path, json).unwrap();

    println!("[GRAPH STORE] persisted {}", graph.graph_id);
}
