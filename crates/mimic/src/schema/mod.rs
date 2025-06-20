pub mod build;
pub mod node;
pub mod types;
pub mod visit;

pub use build::get_schema;

use crate::{
    ThisError,
    schema::{build::BuildError, node::NodeError},
};

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
