pub mod build;
pub mod node;
pub mod visit;

pub mod types {
    pub use mimic_common::schema::types::*;
}

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
