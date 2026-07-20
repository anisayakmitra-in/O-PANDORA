//! Hierarchical Memory — layered memory for Pandora agents.
//!
//! Separates memory into layers with different retention, permissions,
//! and lifetimes. Each layer builds on the one below it:
//!
//!   Global → Organization → Project → Workspace → Session → Execution
//!
//! All search functions return owned entries to avoid borrow conflicts.
//! Inspired by serena's hierarchical memory + mercury's Second Brain.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum MemoryLayer {
    Global,
    Organization,
    Project,
    Workspace,
    Session,
    Execution,
}

impl MemoryLayer {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::Organization => "organization",
            Self::Project => "project",
            Self::Workspace => "workspace",
            Self::Session => "session",
            Self::Execution => "execution",
        }
    }

    pub fn default_ttl(&self) -> Option<u64> {
        match self {
            Self::Global => None,
            Self::Organization => None,
            Self::Project => Some(30 * 24 * 3600),
            Self::Workspace => Some(7 * 24 * 3600),
            Self::Session => Some(24 * 3600),
            Self::Execution => Some(3600),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub layer: MemoryLayer,
    pub content: String,
    pub tags: Vec<String>,
    pub importance: f32,
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub access_count: u64,
    pub pinned: bool,
}

impl MemoryEntry {
    fn touch(&mut self) {
        self.last_accessed = SystemTime::now();
        self.access_count += 1;
    }

    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.layer.default_ttl() {
            let elapsed = SystemTime::now()
                .duration_since(self.created_at)
                .unwrap_or_default()
                .as_secs();
            elapsed > ttl && !self.pinned
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HierarchicalMemory {
    entries: HashMap<String, MemoryEntry>,
    order: Vec<String>, // insertion order for iteration
    next_id: u64,
}

impl HierarchicalMemory {
    pub fn new() -> Self {
        Self::default()
    }

    fn next_id(&mut self) -> String {
        self.next_id += 1;
        format!("mem-{}", self.next_id)
    }

    pub fn remember(
        &mut self,
        layer: MemoryLayer,
        content: String,
        tags: Vec<String>,
        importance: f32,
    ) -> String {
        let id = self.next_id();
        let now = SystemTime::now();
        self.order.push(id.clone());
        self.entries.insert(
            id.clone(),
            MemoryEntry {
                id: id.clone(),
                layer,
                content,
                tags,
                importance: importance.clamp(0.0, 1.0),
                created_at: now,
                last_accessed: now,
                access_count: 0,
                pinned: false,
            },
        );
        id
    }

    /// Returns a clone to avoid borrow conflicts.
    pub fn recall(&mut self, id: &str) -> Option<MemoryEntry> {
        self.entries.get_mut(id).map(|e| {
            e.touch();
            e.clone()
        })
    }

    /// Search by tags. Returns owned entries.
    pub fn search_by_tags(
        &mut self,
        tags: &[&str],
        layer: Option<MemoryLayer>,
    ) -> Vec<MemoryEntry> {
        // Collect matching IDs first
        let mut matches: Vec<(String, f32)> = self
            .entries
            .iter()
            .filter(|(_, e)| {
                let layer_match = layer.is_none_or(|l| e.layer == l);
                let tag_match = tags.iter().any(|t| e.tags.iter().any(|et| et == t));
                layer_match && tag_match
            })
            .map(|(id, e)| (id.clone(), e.importance))
            .collect();

        matches.sort_by_key(|(_, imp)| -(imp * 100.0) as i64);

        // Touch originals and collect results
        let mut results = Vec::with_capacity(matches.len());
        for (id, _) in &matches {
            if let Some(entry) = self.entries.get_mut(id) {
                entry.touch();
                results.push(entry.clone());
            }
        }
        results
    }

    /// Search by content substring. Returns owned entries.
    pub fn search_by_content(
        &mut self,
        query: &str,
        layer: Option<MemoryLayer>,
    ) -> Vec<MemoryEntry> {
        let query = query.to_lowercase();
        let mut matches: Vec<(String, f32)> = self
            .entries
            .iter()
            .filter(|(_, e)| {
                let layer_match = layer.is_none_or(|l| e.layer == l);
                let hit = e.content.to_lowercase().contains(&query)
                    || e.tags.iter().any(|t| t.to_lowercase().contains(&query));
                layer_match && hit
            })
            .map(|(id, e)| (id.clone(), e.importance))
            .collect();

        matches.sort_by_key(|(_, imp)| -(imp * 100.0) as i64);

        let mut results = Vec::with_capacity(matches.len());
        for (id, _) in &matches {
            if let Some(entry) = self.entries.get_mut(id) {
                entry.touch();
                results.push(entry.clone());
            }
        }
        results
    }

    /// Get all entries in a specific layer (owned).
    pub fn layer_entries(&self, layer: MemoryLayer) -> Vec<MemoryEntry> {
        self.entries
            .values()
            .filter(|e| e.layer == layer)
            .cloned()
            .collect()
    }

    pub fn purge_expired(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, e| !e.is_expired());
        self.order.retain(|id| self.entries.contains_key(id));
        before - self.entries.len()
    }

    pub fn pin(&mut self, id: &str) -> bool {
        self.entries
            .get_mut(id)
            .map(|e| {
                e.pinned = true;
            })
            .is_some()
    }

    pub fn forget(&mut self, id: &str) -> bool {
        self.order.retain(|x| x != id);
        self.entries.remove(id).is_some()
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remember_and_recall() {
        let mut m = HierarchicalMemory::new();
        let id = m.remember(
            MemoryLayer::Session,
            "User prefers Rust".into(),
            vec!["pref".into(), "lang".into()],
            0.7,
        );
        let e = m.recall(&id).unwrap();
        assert_eq!(e.layer, MemoryLayer::Session);
        assert_eq!(e.access_count, 1);
    }

    #[test]
    fn search_by_tags_returns_ordered() {
        let mut m = HierarchicalMemory::new();
        m.remember(
            MemoryLayer::Session,
            "cargo build".into(),
            vec!["rust".into(), "build".into()],
            0.3,
        );
        m.remember(
            MemoryLayer::Session,
            "secure coding".into(),
            vec!["rust".into(), "security".into()],
            0.9,
        );
        let results = m.search_by_tags(&["rust"], Some(MemoryLayer::Session));
        assert_eq!(results.len(), 2);
        assert!(results[0].importance >= results[1].importance);
    }

    #[test]
    fn layer_isolation() {
        let mut m = HierarchicalMemory::new();
        m.remember(
            MemoryLayer::Global,
            "global fact".into(),
            vec!["g".into()],
            1.0,
        );
        m.remember(
            MemoryLayer::Session,
            "session fact".into(),
            vec!["s".into()],
            1.0,
        );
        assert_eq!(m.layer_entries(MemoryLayer::Global).len(), 1);
        assert_eq!(m.layer_entries(MemoryLayer::Session).len(), 1);
    }

    #[test]
    fn forget_removes() {
        let mut m = HierarchicalMemory::new();
        let id = m.remember(MemoryLayer::Execution, "temp".into(), vec![], 0.3);
        assert_eq!(m.count(), 1);
        assert!(m.forget(&id));
        assert_eq!(m.count(), 0);
    }

    #[test]
    fn pin_prevents_expiry() {
        let mut m = HierarchicalMemory::new();
        let id = m.remember(MemoryLayer::Execution, "keep me".into(), vec![], 0.9);
        m.pin(&id);
        m.purge_expired();
        assert!(m.recall(&id).is_some());
    }
}
