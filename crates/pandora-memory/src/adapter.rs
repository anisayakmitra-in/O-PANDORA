//! Built-in storage adapters.
//!
//! This module ships with a minimal, in-process storage adapter so
//! the contract crate is exercisable on its own (tests, examples,
//! ephemeral state). Production-grade adapters for SQL, vector
//! stores, KV stores, and graph stores are intentionally out of
//! scope — they live in dedicated adapter crates.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::RwLock;

use crate::error::Result;
use crate::storage::Storage;
use crate::types::{BackendId, StorageId, StorageMetadata};

/// In-process, async storage adapter. Records are kept in a
/// `HashMap` behind an `RwLock`; nothing is persisted to disk.
///
/// This is the simplest possible implementation of [`Storage`].
/// It is suitable for tests, ephemeral state, and as a reference
/// for adapter authors. Production subsystems should use a real
/// adapter (SQL, vector, KV, etc.).
pub struct InMemoryStorage {
    backend_id: BackendId,
    records: Arc<RwLock<HashMap<StorageId, Value>>>,
    metadata: Arc<RwLock<HashMap<StorageId, StorageMetadata>>>,
}

impl InMemoryStorage {
    /// Create a new in-memory storage backend with the given id.
    pub fn new(backend_id: impl Into<BackendId>) -> Self {
        Self {
            backend_id: backend_id.into(),
            records: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Storage for InMemoryStorage {
    fn backend_id(&self) -> BackendId {
        self.backend_id.clone()
    }

    async fn put(&self, id: StorageId, value: Value) -> Result<()> {
        self.records.write().await.insert(id, value);
        Ok(())
    }

    async fn get(&self, id: &str) -> Result<Option<Value>> {
        Ok(self.records.read().await.get(id).cloned())
    }

    async fn exists(&self, id: &str) -> Result<bool> {
        Ok(self.records.read().await.contains_key(id))
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let removed = self.records.write().await.remove(id).is_some();
        self.metadata.write().await.remove(id);
        Ok(removed)
    }

    async fn list_ids(&self) -> Result<Vec<StorageId>> {
        Ok(self.records.read().await.keys().cloned().collect())
    }

    async fn put_metadata(&self, id: &str, metadata: &StorageMetadata) -> Result<()> {
        self.metadata
            .write()
            .await
            .insert(id.to_string(), metadata.clone());
        Ok(())
    }

    async fn get_metadata(&self, id: &str) -> Result<Option<StorageMetadata>> {
        Ok(self.metadata.read().await.get(id).cloned())
    }
}

/// Convenience constructor: `Arc<InMemoryStorage>` as `Arc<dyn Storage>`.
pub fn in_memory(backend_id: impl Into<BackendId>) -> Arc<dyn Storage> {
    Arc::new(InMemoryStorage::new(backend_id))
}
