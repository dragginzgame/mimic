mod load;

pub use load::*;

use crate::{Error, ThisError, db::DbError};

///
/// ResponseError
///

#[derive(Debug, ThisError)]
pub enum ResponseError {
    #[error("expected one or more rows, found 0 (entity {0})")]
    NoRowsFound(String),
}

impl From<ResponseError> for Error {
    fn from(err: ResponseError) -> Self {
        DbError::from(err).into()
    }
}
