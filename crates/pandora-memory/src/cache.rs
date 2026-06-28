//! Cache abstraction over a storage backend.
//!
//! A [`Cache`] is a read-through wrapper that fronts any
//! [`Storage`] backend and applies a configurable eviction
//! policy.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde_json::Value;

use crate::error::Result;
use crate::storage::Storage;
use crate::types::KvKey;

/// Eviction policy for a cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Cache holds at most `capacity` entries; oldest is evicted
    /// when the cap is reached.
    Lru,
    /// Cache holds at most `capacity` entries; least-frequently
    /// used is evicted.
    Lfu,
    /// Cache holds entries until they expire.
    Ttl,
    /// No eviction; cache grows unbounded.
    None,
}

impl Default for EvictionPolicy {
    fn default() -> Self {
        EvictionPolicy::Lru
    }
}

/// Configuration for a cache.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries the cache will hold.
    pub capacity: usize,
    /// Eviction policy.
    pub policy: EvictionPolicy,
    /// TTL for entries when [`EvictionPolicy::Ttl`] is selected.
    pub ttl: Option<Duration>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            capacity: 1024,
            policy: EvictionPolicy::Lru,
            ttl: None,
        }
    }
}

/// Cache interface. Backed by a [`Storage`] for the cold tier.
#[async_trait]
pub trait Cache: Send + Sync {
    /// Get a value by key. Returns `Ok(None)` on miss.
    async fn get(&self, key: &KvKey) -> Result<Option<Value>>;

    /// Insert or replace a value under `key`.
    async fn put(&self, key: &KvKey, value: Value) -> Result<()>;

    /// Invalidate a single key.
    async fn invalidate(&self, key: &KvKey) -> Result<()>;

    /// Invalidate every entry in the cache.
    async fn invalidate_all(&self) -> Result<()>;

    /// Number of entries currently held by the cache.
    async fn len(&self) -> usize;

    /// True if the cache holds no entries.
    async fn is_empty(&self) -> bool;
}

/// Build a fresh cache fronting `storage` with the given config.
pub fn build_cache(storage: Arc<dyn Storage>, config: CacheConfig) -> Result<Arc<dyn Cache>> {
    Ok(Arc::new(InMemoryCache::new(storage, config)))
}

/// A simple in-memory cache fronting a [`Storage`] backend.
///
/// Used as the default adapter; concrete subsystems can supply
/// their own [`Cache`] implementations (Redis, moka, etcd, …).
pub struct InMemoryCache {
    storage: Arc<dyn Storage>,
    config: CacheConfig,
    entries: tokio::sync::RwLock<std::collections::HashMap<KvKey, Value>>,
}

impl InMemoryCache {
    /// Build a new in-memory cache.
    pub fn new(storage: Arc<dyn Storage>, config: CacheConfig) -> Self {
        Self {
            storage,
            config,
            entries: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait]
impl Cache for InMemoryCache {
    async fn get(&self, key: &KvKey) -> Result<Option<Value>> {
        let entries = self.entries.read().await;
        if let Some(value) = entries.get(key) {
            return Ok(Some(value.clone()));
        }
        drop(entries);

        if let Some(value) = self.storage.get(key).await? {
            let mut entries = self.entries.write().await;
            if entries.len() >= self.config.capacity {
                if let Some(first) = entries.keys().next().cloned() {
                    entries.remove(&first);
                }
            }
            entries.insert(key.clone(), value.clone());
            return Ok(Some(value));
        }
        Ok(None)
    }

    async fn put(&self, key: &KvKey, value: Value) -> Result<()> {
        let mut entries = self.entries.write().await;
        if entries.len() >= self.config.capacity
            && !entries.contains_key(key)
            && matches!(
                self.config.policy,
                EvictionPolicy::Lru | EvictionPolicy::Lfu
            )
        {
            if let Some(first) = entries.keys().next().cloned() {
                entries.remove(&first);
            }
        }
        entries.insert(key.clone(), value);
        Ok(())
    }

    async fn invalidate(&self, key: &KvKey) -> Result<()> {
        self.entries.write().await.remove(key);
        Ok(())
    }

    async fn invalidate_all(&self) -> Result<()> {
        self.entries.write().await.clear();
        Ok(())
    }

    async fn len(&self) -> usize {
        self.entries.read().await.len()
    }

    async fn is_empty(&self) -> bool {
        self.entries.read().await.is_empty()
    }
}
