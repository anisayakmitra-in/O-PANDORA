//! Pandora Vector Store — extracted from pandora-runtime (Phase 1B).
//!
use serde::{Deserialize, Serialize};

use std::fs;

use crate::semantic_memory::MemoryChunk;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabase {
    pub memories: Vec<MemoryChunk>,
}

pub struct VectorStore;

impl VectorStore {
    pub fn save(path: &str, database: &VectorDatabase) -> bool {
        println!("[VECTOR] saving database {}", path);

        let serialized = match serde_json::to_string_pretty(database) {
            Ok(data) => data,

            Err(error) => {
                println!("[VECTOR] serialization error {}", error);

                return false;
            }
        };

        fs::write(path, serialized).is_ok()
    }

    pub fn load(path: &str) -> Option<VectorDatabase> {
        println!("[VECTOR] loading database {}", path);

        let content = fs::read_to_string(path).ok()?;

        serde_json::from_str(&content).ok()
    }
}
