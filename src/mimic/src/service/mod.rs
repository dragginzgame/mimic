pub mod storage;

use storage::StorageError;
use thiserror::Error as ThisError;

///
/// ServiceError
///

#[derive(Debug, ThisError)]
pub enum ServiceError {
    #[error(transparent)]
    StorageError(#[from] StorageError),
}
