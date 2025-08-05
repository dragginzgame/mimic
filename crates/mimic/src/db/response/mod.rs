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
    #[error("no rows found")]
    NoRowsFound,
}
