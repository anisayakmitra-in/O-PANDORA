use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: String,

    pub session_id: String,

    pub timestamp: String,

    pub gene: String,

    pub harness: String,

    pub model: String,

    pub prompt: String,

    pub response: String,

    pub memory_layer: String,

    pub related_memories: Vec<String>,

    pub embedding: Vec<f32>,

    pub salience: f32,

    pub tags: Vec<String>,

    pub memory_type: MemoryType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    Episodic,

    Semantic,

    Procedural,

    Reflection,

    ToolUse,

    Conversation,
}

use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
};

pub fn store_memory(memory: &MemoryRecord) {
    let serialized = serde_json::to_string(memory).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("memory/memory.jsonl")
        .unwrap();

    writeln!(file, "{}", serialized).unwrap();
}

pub fn load_memories() -> Vec<MemoryRecord> {
    let path = "memory/memory.jsonl";

    let contents = read_to_string(path).unwrap_or_default();

    contents
        .lines()
        .filter_map(|line| serde_json::from_str::<MemoryRecord>(line).ok())
        .collect()
}

pub fn summarize_memories(memories: &[MemoryRecord]) -> String {
    if memories.is_empty() {
        return "No memories stored.".to_string();
    }

    let latest = memories.last().unwrap();

    #[allow(dead_code)]
    pub fn export_memories(memories: &[MemoryRecord]) {
        std::fs::create_dir_all("memory/export").unwrap();

        let serialized = serde_json::to_string_pretty(memories).unwrap();

        std::fs::write("memory/export/anubis-export.json", serialized).unwrap();
    }

    #[allow(dead_code)]
    pub fn import_memories() -> Vec<MemoryRecord> {
        let contents =
            std::fs::read_to_string("memory/export/anubis-export.json").unwrap_or_default();

        serde_json::from_str(&contents).unwrap_or_default()
    }

    format!(
        r#"ANUBIS MEMORY SUMMARY

TOTAL MEMORIES: {}

LATEST SESSION: {}

LATEST GENE: {}

LATEST HARNESS: {}

LATEST MODEL: {}

LATEST PROMPT:
{}

MEMORY LAYER:
{}

RELATED MEMORIES:
{}"#,
        memories.len(),
        latest.session_id,
        latest.gene,
        latest.harness,
        latest.model,
        latest.prompt,
        latest.memory_layer,
        latest.related_memories.len(),
    )
}

pub fn export_memories(memories: &[MemoryRecord]) {
    std::fs::create_dir_all("memory/export").unwrap();

    let serialized = serde_json::to_string_pretty(memories).unwrap();

    std::fs::write("memory/export/anubis-export.json", serialized).unwrap();
}

pub fn import_memories() -> Vec<MemoryRecord> {
    let contents = std::fs::read_to_string("memory/export/anubis-export.json").unwrap_or_default();

    serde_json::from_str(&contents).unwrap_or_default()
}
