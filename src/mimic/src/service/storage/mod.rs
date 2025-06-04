mod delete;
mod load;
mod resolver;
mod save;

pub mod macros;
pub mod types;

pub use delete::*;
pub use load::*;
pub use resolver::*;
pub use save::*;

use crate::{SerializeError, ValidationError, db::DbError};
use thiserror::Error as ThisError;

///
/// StorageError
///

#[derive(Debug, ThisError)]
pub enum StorageError {
    #[error("index key error")]
    IndexKeyError,

    #[error("selector not suppoorted")]
    SelectorNotSupported,

    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    SaveError(#[from] SaveError),

    #[error(transparent)]
    ResolverError(#[from] ResolverError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),
}

///
/// DebugContext
///

#[derive(Clone, Debug, Default)]
pub struct DebugContext {
    enabled: bool,
}

impl DebugContext {
    pub const fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn println(&self, s: &str) {
        if self.enabled {
            icu::ic::println!("{s}");
        }
    }
}
