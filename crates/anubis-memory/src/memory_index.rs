use crate::memory_entry::MemoryEntry;

#[derive(Debug, Clone)]
pub struct MemoryIndex {
    pub entries: Vec<MemoryEntry>,
}

impl MemoryIndex {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, entry: MemoryEntry) {
        self.entries.push(entry);
    }

    pub fn search(&self, query: &str) -> Vec<MemoryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.content.contains(query))
            .cloned()
            .collect()
    }
}
