pub mod key;
pub mod reference;
pub mod traits;
pub mod types;
pub mod validate;
pub mod value;
pub mod visit;

pub use key::Key;
pub use reference::Reference;
pub use value::{Value, ValueMap};

use thiserror::Error as ThisError;
use validate::ValidateError;

///
/// CoreError
///

#[derive(Debug, ThisError)]
pub enum CoreError {
    #[error(transparent)]
    ValidateError(ValidateError),
}
