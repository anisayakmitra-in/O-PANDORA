use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTrace {
    pub memory: String,

    pub importance: f32,

    pub frequency: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedMemory {
    pub memory: String,

    pub strength: f32,
}

pub struct MemoryConsolidationEngine;

impl MemoryConsolidationEngine {
    pub fn consolidate(traces: &[MemoryTrace]) -> Vec<ConsolidatedMemory> {
        let mut memories = Vec::new();

        for trace in traces {
            let strength = trace.importance * trace.frequency as f32;

            println!("[CONSOLIDATION] {} strength={}", trace.memory, strength);

            if strength > 2.0 {
                memories.push(ConsolidatedMemory {
                    memory: trace.memory.clone(),

                    strength,
                });
            }
        }

        memories
    }
}
