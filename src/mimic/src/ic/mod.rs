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

pub mod helper;
pub mod serialize;
pub mod structures;

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

///
/// CYCLES
///

// Cycle Constants
pub const KC: u128 = 1_000;
pub const MC: u128 = 1_000_000;
pub const BC: u128 = 1_000_000_000;
pub const TC: u128 = 1_000_000_000_000;
pub const QC: u128 = 1_000_000_000_000_000;

// format_tc
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn format_cycles(cycles: u128) -> String {
    format!("{:.6} TC", cycles as f64 / TC as f64)
}
