//! Generic, serializable identifiers used by every storage abstraction.

use serde::{Deserialize, Serialize};

/// Opaque, type-erased identifier for a stored record.
///
/// Concrete subsystems (memory records, genes, audits, etc.) can
/// wrap their own typed newtypes around `StorageId` if they want
/// stronger typing at call sites, or use it directly when the
/// type is irrelevant.
pub type StorageId = String;

/// Identifier for a named storage backend (e.g. `memory-jsonl`,
/// `sqlite-local`, `qdrant-remote`).
pub type BackendId = String;

/// Identifier for a named repository.
pub type RepositoryId = String;

/// Identifier for a snapshot.
pub type SnapshotId = String;

/// Identifier for a transaction.
pub type TransactionId = u64;

/// Key used by key-value stores. Empty keys are reserved for
/// "no key" (singleton records).
pub type KvKey = String;

/// Coarse backend category. New categories can be added without
/// breaking existing adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendKind {
    /// In-process memory (lost on restart). For tests and ephemeral state.
    Memory,
    /// Flat-file or JSONL on local disk.
    File,
    /// SQL database (Postgres, SQLite, MySQL).
    Sql,
    /// Remote or embedded vector database (Qdrant, Milvus, etc.).
    Vector,
    /// Remote object storage (S3, GCS, MinIO).
    Object,
    /// Generic key-value store (Redis, FoundationDB, etc.).
    KeyValue,
    /// Generic graph store (Neo4j, Memgraph, etc.).
    Graph,
    /// Backend does not fit any other category.
    Other,
}

impl Default for BackendKind {
    fn default() -> Self {
        BackendKind::Memory
    }
}

/// Free-form metadata attached to a storage operation, snapshot,
/// or backend instance.
pub type StorageMetadata = std::collections::BTreeMap<String, String>;
