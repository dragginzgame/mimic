pub mod actor;
pub mod macros;

use thiserror::Error as ThisError;

///
/// BuildError
///

#[derive(Debug, ThisError)]
pub enum BuildError {
    #[error("canister path not found: {0}")]
    CanisterNotFound(String),
}
