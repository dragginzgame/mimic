pub mod hash;
pub mod key;
pub mod serialize;
pub mod traits;
pub mod types;
pub mod value;
pub mod visit;

pub use key::Key;
pub use serialize::{SerializeError, deserialize, serialize};
pub use value::Value;
pub use visit::{ValidateError, sanitize, validate};

use thiserror::Error as ThisError;

///
/// CoreError
///

#[derive(Debug, ThisError)]
pub enum CoreError {
    #[error("{0}")]
    ValidateError(#[from] ValidateError),

    #[error("{0}")]
    SerializeError(#[from] SerializeError),
}
