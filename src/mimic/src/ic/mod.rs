///
/// IMPORT IC CRATES
///
pub use ic_cdk::*;
pub use ic_cdk_timers as timers;

// re-exports
pub mod helper;
pub mod structures;

use serde::{Deserialize, Serialize};
use snafu::Snafu;

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

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    CellError { source: structures::CellError },
}
