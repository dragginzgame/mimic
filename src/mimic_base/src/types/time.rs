pub use crate::types;
use mimic::orm::prelude::*;

///
/// Duration
///
/// Duration in seconds
///

#[newtype(primitive = "U64", value(item(is = "types::U64")))]
pub struct Duration {}

///
/// DurationMilli
///
/// Duration in milliseconds
///

#[newtype(primitive = "U64", value(item(is = "types::U64")))]
pub struct DurationMilli {}

///
/// Timestamp
///

#[newtype(primitive = "U64", value(item(is = "types::U64")))]
pub struct Timestamp {}

impl Timestamp {
    #[must_use]
    pub fn now() -> Self {
        Self(*mimic::types::Timestamp::now())
    }
}

///
/// Created
///
/// A Timestamp that defaults to the current now() time
/// when it is created
///

#[newtype(
    primitive = "U64",
    value(
        item(is = "types::time::Timestamp"),
        default = "types::time::Timestamp::now"
    )
)]
pub struct Created {}
