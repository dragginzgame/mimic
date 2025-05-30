pub mod build;
pub mod node;
pub mod state;
pub mod visit;

pub use build::get_schema;

use crate::{
    ThisError,
    schema::{build::BuildError, node::NodeError, state::StateError},
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
