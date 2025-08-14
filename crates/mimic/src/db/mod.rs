pub mod executor;
pub mod query;
pub mod response;
pub mod service;
pub mod store;

use crate::{
    Error,
    core::{serialize::SerializeError, traits::EntityKind, validate::ValidationError},
    db::{
        executor::{DeleteExecutor, ExecutorError, LoadExecutor, SaveExecutor},
        query::QueryError,
        response::ResponseError,
        store::{DataStoreRegistryLocal, IndexStoreRegistryLocal, StoreError},
    },
};
use thiserror::Error as ThisError;

///
/// DbError
///

#[derive(Debug, ThisError)]
pub enum DbError {
    #[error(transparent)]
    ExecutorError(#[from] ExecutorError),

    #[error(transparent)]
    QueryError(#[from] QueryError),

    #[error(transparent)]
    ResponseError(#[from] ResponseError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    StoreError(#[from] StoreError),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),
}

///
/// Db
/// entry point into the whole db crate
///

#[derive(Clone, Copy)]
pub struct Db {
    data: DataStoreRegistryLocal,
    index: IndexStoreRegistryLocal,
}

impl Db {
    #[must_use]
    pub const fn new(data: DataStoreRegistryLocal, index: IndexStoreRegistryLocal) -> Self {
        Self { data, index }
    }

    #[must_use]
    pub const fn load<E: EntityKind>(&self) -> LoadExecutor<E> {
        LoadExecutor::new(self.data, self.index)
    }

    #[must_use]
    pub const fn save<E: EntityKind>(&self) -> SaveExecutor<E> {
        SaveExecutor::new(self.data, self.index)
    }

    pub fn create<E: EntityKind>(&self, entity: E) -> Result<E, Error> {
        SaveExecutor::new(self.data, self.index).create(entity)
    }

    pub fn replace<E: EntityKind>(&self, entity: E) -> Result<E, Error> {
        SaveExecutor::new(self.data, self.index).replace(entity)
    }

    pub fn update<E: EntityKind>(&self, entity: E) -> Result<E, Error> {
        SaveExecutor::new(self.data, self.index).update(entity)
    }

    #[must_use]
    pub const fn delete<E: EntityKind>(&self) -> DeleteExecutor<E> {
        DeleteExecutor::new(self.data, self.index)
    }
}
