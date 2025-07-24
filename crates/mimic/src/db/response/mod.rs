mod load;
mod types;

pub use load::*;
pub use types::*;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// ResponseError
///

#[derive(Debug, ThisError)]
pub enum ResponseError {
    #[error("no rows found")]
    NoRowsFound,
}

///
/// LoadResponse
///
/// this should be CandidType like the other responses, but we're not using it
/// for now as there's no API standard for returning load results specified
/// (over the wire we can't use LoadCollection<E>)
///

#[derive(CandidType, Debug, Deserialize, Serialize)]
pub struct LoadResponse {}
