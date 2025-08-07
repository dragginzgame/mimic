mod load;
mod types;

pub use load::*;
pub use types::*;

use thiserror::Error as ThisError;

///
/// ResponseError
///

#[derive(Debug, ThisError)]
pub enum ResponseError {
    #[error("no rows returned from query (entity {0})")]
    NoRowsFound(String),
}
