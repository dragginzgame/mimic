pub mod query;
// event and snapshot endpoints now call modules directly via codegen; query helpers remain

use thiserror::Error as ThisError;

///
/// InterfaceError
///

#[derive(Debug, ThisError)]
pub enum InterfaceError {
    #[error(transparent)]
    QueryError(#[from] query::QueryError),
}
