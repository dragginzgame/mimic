pub mod metrics;
pub mod query;
pub mod storage;

use thiserror::Error as ThisError;

///
/// InterfaceError
///

#[derive(Debug, ThisError)]
pub enum InterfaceError {
    #[error(transparent)]
    QueryError(#[from] query::QueryError),
}
