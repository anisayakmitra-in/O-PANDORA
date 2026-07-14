use crate::storage::MemoryRecord;

pub fn compress_memory(memory: &MemoryRecord) -> String {
    format!("{} :: {}", memory.prompt, memory.response)
}
