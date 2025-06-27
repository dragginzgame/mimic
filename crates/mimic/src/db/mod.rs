pub mod executor;
pub mod hasher;
pub mod query;
pub mod response;
pub mod service;
pub mod store;

use crate::{
    db::{
        executor::{DeleteExecutor, ExecutorError, LoadExecutor, SaveExecutor},
        query::QueryError,
        response::ResponseError,
        store::{DataStoreRegistry, IndexStoreRegistry, StoreError},
    },
    ops::{serialize::SerializeError, validate::ValidateError},
};
use thiserror::Error as ThisError;

///
/// DataError
///

#[derive(Debug, ThisError)]
pub enum DataError {
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
    pub fn new(data: DataStoreRegistry, index: IndexStoreRegistry) -> Self {
        Self { data, index }
    }

    #[must_use]
    pub fn load(&self) -> LoadExecutor {
        LoadExecutor::new(self.data, self.index)
    }

    #[must_use]
    pub fn save(&self) -> SaveExecutor {
        SaveExecutor::new(self.data, self.index)
    }

    #[must_use]
    pub fn delete(&self) -> DeleteExecutor {
        DeleteExecutor::new(self.data, self.index)
    }
}
