pub use crate::orm::{base::types, prelude::*};

///
/// Duration
///
/// Duration in seconds
///

#[newtype(primitive = "U64", value(item(is = "types::U64")))]
pub struct Duration {}

///
/// DurationMs
///
/// Duration in milliseconds
///

#[newtype(primitive = "U64", value(item(is = "types::U64")))]
pub struct DurationMs {}

impl DurationMs {
    #[must_use]
    pub const fn hour(n: usize) -> Self {
        Self((n * 3_600_000) as u64)
    }
}

///
/// Timestamp
///

#[newtype(primitive = "U64", value(item(is = "types::U64")))]
pub struct Timestamp {}

impl Timestamp {
    #[must_use]
    pub fn now() -> Self {
        Self(crate::utils::time::now_secs())
    }
}

///
/// Created
///
/// A Timestamp that defaults to the current now() time when it is created
///

#[newtype(
    primitive = "U64",
    value(
        item(is = "types::time::Timestamp"),
        default = "types::time::Timestamp::now"
    )
)]
pub struct Created {}
