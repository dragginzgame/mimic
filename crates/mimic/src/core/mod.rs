pub mod db;
pub mod traits;
pub mod types;
pub mod validate;
pub mod value;
pub mod visit;

use thiserror::Error as ThisError;
use validate::ValidateError;
use value::ValueError;

///
/// CoreError
///

#[derive(Debug, ThisError)]
pub enum CoreError {
    #[error(transparent)]
    ValidateError(ValidateError),

    #[error(transparent)]
    ValueError(ValueError),
}
