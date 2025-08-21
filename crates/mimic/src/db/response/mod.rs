mod load;

pub use load::*;

use thiserror::Error as ThisError;

///
/// ResponseError
///

#[derive(Debug, ThisError)]
pub enum ResponseError {
    #[error("expected one or more rows, found 0 (entity {0})")]
    NoRowsFound(String),
}
