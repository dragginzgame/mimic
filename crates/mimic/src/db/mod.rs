pub mod executor;
pub mod hasher;
pub mod query;
pub mod response;
pub mod service;
pub mod store;

use crate::{
    core::{serialize::SerializeError, validate::ValidationError},
    db::{
        executor::{DeleteExecutor, ExecutorError, LoadExecutor, SaveExecutor},
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

    // specific save queries
    #[must_use]
    pub const fn create(&self) -> SaveExecutor {
        SaveExecutor::new(self.data, self.index)
    }
}
