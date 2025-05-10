pub mod serialize;
pub mod structures;

///
/// IMPORT IC CRATES
///

pub mod api {
    pub use ic_cdk::api::*;
}
pub mod mgmt {
    pub use ic_cdk::management_canister::*;
}
pub use ic_cdk::*;
pub use serialize::{deserialize, serialize};

use crate::ThisError;
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// IcError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum IcError {
    #[error(transparent)]
    CellError(#[from] structures::cell::CellError),
}
