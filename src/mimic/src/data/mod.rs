pub mod executor;
pub mod query;
pub mod response;
pub mod store;

use crate::{SerializeError, ValidationError};
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
    ResolverError(#[from] executor::ResolverError),

    #[error(transparent)]
    ResponseError(#[from] response::ResponseError),

    #[error(transparent)]
    StoreError(#[from] store::StoreError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),
}
