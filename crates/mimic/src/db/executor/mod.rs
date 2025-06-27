mod delete;
mod load;
mod save;
mod types;

pub use delete::*;
pub use load::*;
pub use save::*;
pub use types::*;

use crate::db::store::{DataKey, IndexKey};
use thiserror::Error as ThisError;

///
/// ExecutorError
///

#[derive(Debug, ThisError)]
pub enum ExecutorError {
    #[error("data key exists: {0}")]
    KeyExists(DataKey),

    #[error("data key not found: {0}")]
    KeyNotFound(DataKey),

    #[error("index constraint violation for index: {0:?}")]
    IndexViolation(IndexKey),
}
