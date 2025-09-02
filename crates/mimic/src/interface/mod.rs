pub mod query;
pub mod metrics;

use thiserror::Error as ThisError;

///
/// InterfaceError
///

#[derive(Debug, ThisError)]
pub enum InterfaceError {
    #[error(transparent)]
    QueryError(#[from] query::QueryError),
}
