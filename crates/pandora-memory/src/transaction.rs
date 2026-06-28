//! Transaction abstractions.
//!
//! A [`Transaction`] is a single unit of work that either commits
//! all of its writes or aborts them. The trait is intentionally
//! minimal: each backend adapter decides how to implement
//! isolation, locking, and rollback.

use async_trait::async_trait;

use crate::error::Result;
use crate::types::TransactionId;

/// A unit of work against a storage backend.
#[async_trait]
pub trait Transaction: Send + Sync {
    /// Transaction id assigned by the backend.
    fn id(&self) -> TransactionId;

    /// True if the transaction has already been committed or
    /// aborted. No further operations are valid once the
    /// transaction is finalized.
    fn is_finalized(&self) -> bool;

    /// Commit all writes performed inside this transaction.
    async fn commit(self: Box<Self>) -> Result<()>;

    /// Abort and discard all writes performed inside this
    /// transaction.
    async fn abort(self: Box<Self>) -> Result<()>;
}

/// Factory for opening new transactions on a backend.
#[async_trait]
pub trait Transactional: Send + Sync {
    /// Begin a new transaction. The default implementation in
    /// adapters may choose any isolation level appropriate for
    /// the backend.
    async fn begin(&self) -> Result<Box<dyn Transaction>>;
}
