pub mod executor;
pub mod hasher;
pub mod query;
pub mod response;
pub mod service;
pub mod store;
pub mod types;

use crate::{
    db::{
        executor::{DeleteExecutor, LoadExecutor, LoadExecutorDyn, SaveExecutor},
        store::{DataStoreRegistry, IndexStoreRegistry},
    },
    def::{SerializeError, ValidationError},
};
use thiserror::Error as ThisError;

///
/// DataError
///

#[derive(Debug, ThisError)]
pub enum DataError {
    #[error(transparent)]
    ExecutorError(#[from] executor::ExecutorError),

    #[error(transparent)]
    QueryError(#[from] query::QueryError),

    #[error(transparent)]
    ResponseError(#[from] response::ResponseError),

    #[error(transparent)]
    StoreError(#[from] store::StoreError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),
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
    pub fn load_dyn(&self) -> LoadExecutorDyn {
        LoadExecutorDyn::new(self.data, self.index)
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
