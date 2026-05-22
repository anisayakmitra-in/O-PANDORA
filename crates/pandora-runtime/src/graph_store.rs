use std::fs;

use crate::execution_graph::ExecutionGraph;

pub fn persist_graph(
    graph: &ExecutionGraph,
) {

    fs::create_dir_all(
        "graphs"
    )
    .unwrap();

    let path =
        format!(
            "graphs/{}.json",
            graph.graph_id
        );

    let serialized =
        serde_json::to_string_pretty(
            graph
        )
        .unwrap();

    fs::write(
        path,
        serialized,
    )
    .unwrap();
}
