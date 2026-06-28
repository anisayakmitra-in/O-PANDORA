//! Repository pattern over a [`Storage`] backend.
//!
//! A [`Repository<T>`] is a typed view over a [`Storage`] that
//! knows how to convert records of type `T` to and from
//! `serde_json::Value` and how to compute storage ids for them.
//! The repository is the layer the rest of the workspace
//! interacts with — it never touches the raw storage backend.

use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{Result, StorageError};
use crate::storage::Storage;
use crate::types::{RepositoryId, StorageId};

/// Strategy for turning a record into a storage id.
pub type IdFn<T> = Arc<dyn Fn(&T) -> StorageId + Send + Sync>;

/// Optional validator invoked before a record is written.
pub type ValidateFn<T> = Arc<dyn Fn(&T) -> Result<()> + Send + Sync>;

/// Typed, transactional view over a [`Storage`] backend for
/// records of type `T`.
#[async_trait]
pub trait Repository<T>: Send + Sync
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// Repository identifier.
    fn id(&self) -> RepositoryId;

    /// Insert or replace a record.
    async fn put(&self, value: &T) -> Result<()>;

    /// Fetch a record by its computed id.
    async fn get(&self, record_id: &str) -> Result<Option<T>>;

    /// Return true if a record with the given id exists.
    async fn exists(&self, record_id: &str) -> Result<bool>;

    /// Delete a record by id.
    async fn delete(&self, record_id: &str) -> Result<bool>;

    /// List all record ids in this repository.
    async fn list_ids(&self) -> Result<Vec<StorageId>>;
}

/// Default [`Repository<T>`] implementation backed by a
/// [`Storage`] and an id-extraction function.
pub struct TypedRepository<T> {
    repo_id: RepositoryId,
    storage: Arc<dyn Storage>,
    id_fn: IdFn<T>,
    validate: Option<ValidateFn<T>>,
    _marker: PhantomData<T>,
}

impl<T> TypedRepository<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// Create a new typed repository over `storage`, deriving ids
    /// via `id_fn`.
    pub fn new(
        repo_id: impl Into<RepositoryId>,
        storage: Arc<dyn Storage>,
        id_fn: IdFn<T>,
    ) -> Self {
        Self {
            repo_id: repo_id.into(),
            storage,
            id_fn,
            validate: None,
            _marker: PhantomData,
        }
    }

    /// Attach a validator that runs on every `put`.
    pub fn with_validator(mut self, validate: ValidateFn<T>) -> Self {
        self.validate = Some(validate);
        self
    }
}

#[async_trait]
impl<T> Repository<T> for TypedRepository<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn id(&self) -> RepositoryId {
        self.repo_id.clone()
    }

    async fn put(&self, value: &T) -> Result<()> {
        if let Some(validate) = &self.validate {
            validate(value)?;
        }
        let id = (self.id_fn)(value);
        let value_json =
            serde_json::to_value(value).map_err(|e| StorageError::Serialization(e.to_string()))?;
        self.storage.put(id, value_json).await
    }

    async fn get(&self, record_id: &str) -> Result<Option<T>> {
        match self.storage.get(record_id).await? {
            Some(value) => {
                let typed: T = serde_json::from_value(value)
                    .map_err(|e| StorageError::Serialization(e.to_string()))?;
                Ok(Some(typed))
            }
            None => Ok(None),
        }
    }

    async fn exists(&self, record_id: &str) -> Result<bool> {
        self.storage.exists(record_id).await
    }

    async fn delete(&self, record_id: &str) -> Result<bool> {
        self.storage.delete(record_id).await
    }

    async fn list_ids(&self) -> Result<Vec<StorageId>> {
        self.storage.list_ids().await
    }
}
