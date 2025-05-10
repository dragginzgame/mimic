pub mod query;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// InterfaceError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum InterfaceError {
    #[error(transparent)]
    QueryError(#[from] query::QueryError),
}
