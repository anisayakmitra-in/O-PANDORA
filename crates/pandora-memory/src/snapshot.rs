//! Snapshot abstractions.
//!
//! A snapshot is a point-in-time, immutable view of a storage
//! backend. The trait is the same regardless of whether the
//! snapshot is persisted to disk, copied to a remote object
//! store, or held in memory.

use std::sync::Arc;

use async_trait::async_trait;

use crate::error::Result;
use crate::storage::Storage;
use crate::types::SnapshotId;

/// A captured, immutable point-in-time view of a storage backend.
#[async_trait]
pub trait Snapshot: Send + Sync {
    /// Snapshot identifier assigned at creation.
    fn id(&self) -> SnapshotId;

    /// Human-readable description of what this snapshot contains.
    fn description(&self) -> &str;

    /// When the snapshot was created, in milliseconds since the
    /// UNIX epoch. Zero means "unknown".
    fn created_at_ms(&self) -> u64;

    /// Read-only view into the captured state.
    fn storage(&self) -> Arc<dyn Storage>;
}

/// Factory for capturing snapshots of a storage backend.
#[async_trait]
pub trait Snapshotter: Send + Sync {
    /// Capture a snapshot of `storage` and store it under
    /// `description`.
    async fn snapshot(
        &self,
        storage: Arc<dyn Storage>,
        description: impl Into<String> + Send,
    ) -> Result<Arc<dyn Snapshot>>;

    /// Restore `snapshot` into `target`. Implementations decide
    /// whether this is a full overwrite, a merge, or a no-op for
    /// conflicting records.
    async fn restore(&self, target: Arc<dyn Storage>, snapshot: Arc<dyn Snapshot>) -> Result<()>;

    /// List all snapshots currently held by this snapshotter.
    async fn list_snapshots(&self) -> Result<Vec<SnapshotId>>;
}
