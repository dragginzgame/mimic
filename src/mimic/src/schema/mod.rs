pub mod build;
pub mod node;
pub mod state;
pub mod visit;

use crate::{
    schema::{build::BuildError, state::StateError},
    ThisError,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// SchemaError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum SchemaError {
    #[error("error downcasting schema node: {0}")]
    DowncastFail(String),

    #[error("{0} is an incorrect node type")]
    IncorrectNodeType(String),

    #[error("path not found: {0}")]
    PathNotFound(String),

    #[error(transparent)]
    BuildError(#[from] BuildError),

    #[error(transparent)]
    StateError(#[from] StateError),
}
