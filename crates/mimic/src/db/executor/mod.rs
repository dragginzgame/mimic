mod delete;
mod helper;
mod load;
mod save;

pub use delete::*;
pub use helper::*;
pub use load::*;
pub use save::*;

use crate::db::types::{IndexKey, SortKey};
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
