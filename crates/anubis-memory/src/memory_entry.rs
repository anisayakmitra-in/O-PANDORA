use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub memory_id: String,

    pub namespace: String,

    pub content: String,

    pub tags: Vec<String>,

    pub related_events: Vec<String>,

    pub related_graphs: Vec<String>,

    pub timestamp: String,
}
