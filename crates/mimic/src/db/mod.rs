pub mod executor;
pub mod hasher;
pub mod query;
pub mod response;
pub mod service;
pub mod store;

use crate::{
    MimicError,
    core::{Key, traits::EntityKind, validate::ValidateError},
    db::{
        executor::{DeleteExecutor, ExecutorError, LoadExecutor, SaveExecutor},
        query::QueryError,
        response::{LoadCollection, ResponseError},
        store::{DataStoreRegistry, IndexStoreRegistry, StoreError},
    },
    serialize::SerializeError,
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
    StoreError(#[from] StoreError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    ValidateError(#[from] ValidateError),
}

///
/// Db
/// entry point into the whole db crate
///

pub struct Db {
    data: DataStoreRegistry,
    index: IndexStoreRegistry,
}

impl Db {
    #[must_use]
    pub const fn new(data: DataStoreRegistry, index: IndexStoreRegistry) -> Self {
        Self { data, index }
    }

    #[must_use]
    pub const fn load(&self) -> LoadExecutor {
        LoadExecutor::new(self.data, self.index)
    }

    #[must_use]
    pub const fn save(&self) -> SaveExecutor {
        SaveExecutor::new(self.data, self.index)
    }

    #[must_use]
    pub const fn delete(&self) -> DeleteExecutor {
        DeleteExecutor::new(self.data, self.index)
    }

    //
    // HELPERS
    //

    pub fn load_all<E: EntityKind>(&self) -> Result<LoadCollection<E>, MimicError> {
        LoadExecutor::new(self.data, self.index).execute(query::load().all())
    }

    pub fn load_one<E: EntityKind>(
        &self,
        k: impl Into<Key>,
    ) -> Result<LoadCollection<E>, MimicError> {
        LoadExecutor::new(self.data, self.index).execute(query::load().one(k))
    }
}
