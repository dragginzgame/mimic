pub mod build;
pub mod node;
pub mod types;
pub mod visit;

use crate::{build::BuildError, node::NodeError};
use thiserror::Error as ThisError;

///
/// SchemaError
///

#[derive(Debug, ThisError)]
pub enum SchemaError {
    #[error(transparent)]
    BuildError(#[from] BuildError),

    #[error(transparent)]
    NodeError(#[from] NodeError),
}
