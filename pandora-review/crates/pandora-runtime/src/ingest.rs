use std::fs;

use crate::trace::RuntimeTrace;

pub fn ingest_traces() -> Vec<RuntimeTrace> {
    let mut traces = Vec::new();

    let entries = fs::read_dir("traces").unwrap();

    for entry in entries {
        let path = entry.unwrap().path();

        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if extension == "json" {
                let contents = fs::read_to_string(&path).unwrap();

                let trace = serde_json::from_str::<RuntimeTrace>(&contents).unwrap();

                traces.push(trace);
            }
        }
    }

    traces
}
