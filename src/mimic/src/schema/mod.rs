pub mod build;
pub mod node;
pub mod state;
pub mod visit;

pub use build::get_schema;

use crate::{
    schema::{build::BuildError, node::NodeError, state::StateError},
    ThisError,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// SchemaError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum SchemaError {
    #[error(transparent)]
    BuildError(#[from] BuildError),

    #[error(transparent)]
    NodeError(#[from] NodeError),

    #[error(transparent)]
    StateError(#[from] StateError),
}
