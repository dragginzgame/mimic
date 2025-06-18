mod delete;
mod helper;
mod load;
mod save;

pub use delete::*;
pub use helper::*;
pub use load::*;
pub use save::*;

use crate::db::{
    store::{DataStoreRegistry, IndexStoreRegistry},
    types::{IndexKey, SortKey},
};
use thiserror::Error as ThisError;

///
/// ExecutorError
///

#[derive(Debug, ThisError)]
pub enum ExecutorError {
    #[error("key exists: {0}")]
    KeyExists(SortKey),

    #[error("key not found: {0}")]
    KeyNotFound(SortKey),

    #[error("index constraint violation for index: {0:?}")]
    IndexViolation(IndexKey),
}

///
/// Executor
///

pub struct Executor {
    data: DataStoreRegistry,
    index: IndexStoreRegistry,
}

impl Executor {
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
