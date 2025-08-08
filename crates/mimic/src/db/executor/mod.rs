mod context;
mod delete;
mod filter;
mod load;
mod save;

pub use context::*;
pub use delete::*;
pub use filter::*;
pub use load::*;
pub use save::*;

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

    #[error("index execution not yet supported")]
    IndexNotSupported,

    #[error("index constraint violation for index: {0:?}")]
    IndexViolation(IndexKey),
}
