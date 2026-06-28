//! Storage backend abstraction.
//!
//! The [`Storage`] trait is the single, canonical interface every
//! backend adapter (memory, file, SQL, vector, KV, graph, object)
//! implements. Subsystems above this crate (memory, governance,
//! runtime, etc.) depend on the trait — not on any concrete
//! backend — so the same code can target any backend.
//!
//! The trait is object-safe: methods are typed in terms of
//! `serde_json::Value` rather than a generic `T`. The
//! [`TypedRepository`](crate::repository::TypedRepository) wrapper
//! provides typed access on top of a `dyn Storage`.

use async_trait::async_trait;
use serde_json::Value;

use crate::error::Result;
use crate::types::{BackendId, StorageId, StorageMetadata};

/// Core, backend-agnostic storage interface.
///
/// All methods are async to allow both local (in-process) and
/// remote (network) backends. Concrete implementations live in
/// sub-crates or adapter modules; the trait itself is
/// `dyn`-compatible.
#[async_trait]
pub trait Storage: Send + Sync {
    /// Identifier of this backend instance (e.g. `primary`,
    /// `qdrant-remote`).
    fn backend_id(&self) -> BackendId;

    /// Insert or replace a record under the given id. The value
    /// must already be a `serde_json::Value`; serialization to the
    /// backend format is the adapter's responsibility.
    async fn put(&self, id: StorageId, value: Value) -> Result<()>;

    /// Fetch a record by id. Returns `Ok(None)` when the id is
    /// not present.
    async fn get(&self, id: &str) -> Result<Option<Value>>;

    /// Return true if a record with the given id exists.
    async fn exists(&self, id: &str) -> Result<bool>;

    /// Delete a record by id. Returns true if a record was removed.
    async fn delete(&self, id: &str) -> Result<bool>;

    /// List all record ids in this storage instance. May be
    /// expensive for some backends.
    async fn list_ids(&self) -> Result<Vec<StorageId>>;

    /// Attach free-form metadata to a record. Implementations that
    /// do not support per-record metadata may return
    /// [`crate::error::StorageError::Unsupported`].
    async fn put_metadata(&self, id: &str, metadata: &StorageMetadata) -> Result<()>;

    /// Read the metadata attached to a record. Returns
    /// `Ok(None)` if no metadata is stored.
    async fn get_metadata(&self, id: &str) -> Result<Option<StorageMetadata>>;
}
