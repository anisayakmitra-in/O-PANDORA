//! Pandora Tracing — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    pub trace_id: String,

    pub subsystem: String,

    pub event: String,

    pub timestamp: String,
}

pub struct TraceEngine;

impl TraceEngine {
    pub fn emit(event: &TraceEvent) {
        fs::create_dir_all("traces").unwrap();

        let path = format!("traces/{}.json", event.trace_id);

        let content = serde_json::to_string_pretty(event).unwrap();

        fs::write(path, content).unwrap();

        println!("[TRACE] {} :: {}", event.subsystem, event.event);
    }
}
