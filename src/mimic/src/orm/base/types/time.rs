pub use crate::orm::{base::types, prelude::*};

///
/// Milliseconds
///

#[newtype(primitive = "Nat64", item(is = "types::Nat64"))]
pub struct Milliseconds {}

///
/// Seconds
///

#[newtype(primitive = "Nat64", item(is = "types::Nat64"))]
pub struct Seconds {}

///
/// Minutes
///

#[newtype(primitive = "Nat64", item(is = "types::Nat64"))]
pub struct Minutes {}

///
/// Timestamp
///

#[newtype(primitive = "Nat64", item(is = "types::Nat64"))]
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
    primitive = "Nat64",
    item(is = "types::time::Timestamp"),
    default = "types::time::Timestamp::now"
)]
pub struct Created {}
