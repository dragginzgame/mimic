pub use crate::orm::{base::types, prelude::*};

///
/// Seconds
///

#[newtype(primitive = "U64", value(item(is = "types::U64")))]
pub struct Seconds {}

///
/// Milliseconds
///

#[newtype(primitive = "U64", value(item(is = "types::U64")))]
pub struct Milliseconds {}

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
