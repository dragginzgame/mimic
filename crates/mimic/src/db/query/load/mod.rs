mod dynamic;
mod generic;

pub use dynamic::*;
pub use generic::*;

use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadFormat
///

#[derive(CandidType, Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum LoadFormat {
    #[default]
    Rows,
    Keys,
    Count,
}
