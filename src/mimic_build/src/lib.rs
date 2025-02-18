pub mod actor;
pub mod macros;
pub mod schema;

use actor::ActorError;
use schema::SchemaError;
use serde::Serialize;
use thiserror::Error as ThisError;

///
/// Error
/// top level crate error
///

#[derive(Debug, Serialize, ThisError)]
pub enum Error {
    #[error(transparent)]
    ActorError(#[from] ActorError),

    #[error(transparent)]
    SchemaError(#[from] SchemaError),
}
