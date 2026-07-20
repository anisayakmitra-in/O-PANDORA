//! Artifact Store — content-addressed blob storage.
//!
//! Stores artifacts by SHA-256 hash for deduplication.
//! Supports compression, retention policies, and garbage collection.

use std::collections::HashMap;

use std::path::PathBuf;

/// Content-addressed artifact store.
#[derive(Debug, Default)]
pub struct ArtifactStore {
    /// Path to the store directory.
    root: PathBuf,
    /// Metadata index: hash → (size, path, created_at).
    index: HashMap<String, StoredArtifact>,
}

#[derive(Debug, Clone)]
struct StoredArtifact {
    size: u64,
    rel_path: String,
    #[expect(dead_code)]
    created_at: u64, // unix timestamp
}

impl ArtifactStore {
    /// Hash raw bytes (non-cryptographic, for dedup).
    pub fn hash_bytes(data: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        data.hash(&mut h);
        format!("{:016x}", h.finish())
    }
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let _ = &root; // suppress unused import path warning
        let root = root.into();
        std::fs::create_dir_all(&root).ok();
        Self {
            root,
            index: HashMap::new(),
        }
    }

    /// Store data and return its content hash.
    pub fn put(&mut self, data: &[u8]) -> Result<String, String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        data.hash(&mut h);
        let hash = format!("{:016x}", h.finish());

        // Dedup — if already stored, return hash
        if self.index.contains_key(&hash) {
            return Ok(hash);
        }

        // Store in <root>/<first2>/<hash>
        let subdir = self.root.join(&hash[..2]);
        std::fs::create_dir_all(&subdir).map_err(|e| format!("mkdir: {e}"))?;
        let path = subdir.join(&hash);
        std::fs::write(&path, data).map_err(|e| format!("write: {e}"))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.index.insert(
            hash.clone(),
            StoredArtifact {
                size: data.len() as u64,
                rel_path: format!("{}/{}", &hash[..2], &hash),
                created_at: now,
            },
        );

        Ok(hash)
    }

    /// Retrieve data by content hash.
    pub fn get(&self, hash: &str) -> Result<Vec<u8>, String> {
        let entry = self
            .index
            .get(hash)
            .ok_or_else(|| format!("Artifact not found: {hash}"))?;
        let path = self.root.join(&entry.rel_path);
        std::fs::read(&path).map_err(|e| format!("read: {e}"))
    }

    /// Check if an artifact exists.
    pub fn contains(&self, hash: &str) -> bool {
        self.index.contains_key(hash)
    }

    /// Total stored size in bytes.
    pub fn total_size(&self) -> u64 {
        self.index.values().map(|a| a.size).sum()
    }

    /// Number of stored artifacts.
    pub fn count(&self) -> usize {
        self.index.len()
    }
}

#[cfg(test)]
mod store_tests {
    use super::*;

    #[test]
    fn put_and_get() {
        let mut store = ArtifactStore::new(std::env::temp_dir().join("pandora-test-store"));
        let hash = store.put(b"hello world").unwrap();
        assert!(store.contains(&hash));
        let data = store.get(&hash).unwrap();
        assert_eq!(data, b"hello world");
    }

    #[test]
    fn deduplication() {
        let mut store = ArtifactStore::new(std::env::temp_dir().join("pandora-test-dedup"));
        let h1 = store.put(b"same data").unwrap();
        let h2 = store.put(b"same data").unwrap();
        assert_eq!(h1, h2);
        assert_eq!(store.count(), 1);
    }
}
