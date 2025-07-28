use crate::design::{base::types, prelude::*};

///
/// Milliseconds
///

#[newtype(primitive = "Nat64", item(prim = "Nat64"))]
pub struct Milliseconds {}

///
/// Seconds
///

#[newtype(primitive = "Nat64", item(prim = "Nat64"))]
pub struct Seconds {}

///
/// Minutes
///

#[newtype(primitive = "Nat64", item(prim = "Nat64"))]
pub struct Minutes {}

///
/// Timestamp
///

#[newtype(primitive = "Nat64", item(prim = "Nat64"))]
pub struct Timestamp {}

impl Timestamp {
    #[must_use]
    pub fn now() -> Self {
        Self(icu::utils::time::now_secs())
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

///
/// Duration
///
/// just a quick one, can make it better
/// seconds for now, maybe we need ms?
///

#[newtype(primitive = "Nat64", item(prim = "Nat64"))]
pub struct Duration {}

impl Duration {
    #[must_use]
    pub const fn from_minutes(minutes: u64) -> Self {
        Self(minutes * 60)
    }

    #[must_use]
    pub const fn from_hours(hours: u64) -> Self {
        Self(hours * 3_600)
    }

    #[must_use]
    pub const fn from_days(days: u64) -> Self {
        Self(days * 86_400)
    }

    #[must_use]
    pub const fn as_secs(&self) -> u64 {
        self.0
    }
}
