use crate::design::{base::sanitizer, prelude::*};

///
/// CreatedAt
///

#[newtype(
    primitive = "Timestamp",
    item(prim = "Timestamp"),
    ty(sanitizer(path = "sanitizer::time::CreatedAt"))
)]
pub struct CreatedAt {}

///
/// UpdatedAt
///

#[newtype(
    primitive = "Timestamp",
    item(prim = "Timestamp"),
    ty(sanitizer(path = "sanitizer::time::UpdatedAt"))
)]
pub struct UpdatedAt {}

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
