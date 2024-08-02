pub mod bytes;
pub mod color;
pub mod iso;
pub mod len;
pub mod number;
pub mod string;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("invalid enum variant '{variant}'"))]
    InvalidVariant { variant: String },
}

impl Error {
    #[must_use]
    pub fn invalid_variant(variant: &str) -> Self {
        Self::InvalidVariant {
            variant: variant.to_string(),
        }
    }
}
