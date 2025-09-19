use crate::core::{
    Value,
    traits::{FieldValue, Sanitize, TypeView, ValidateAuto, ValidateCustom, Visitable},
};
use candid::CandidType;
use derive_more::{Add, AddAssign, Display, FromStr, Sub, SubAssign};
use serde::{Deserialize, Serialize};

///
/// Timestamp
/// (in seconds)
///

#[derive(
    Add,
    AddAssign,
    CandidType,
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    Eq,
    FromStr,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    Sub,
    SubAssign,
)]
#[repr(transparent)]
pub struct Timestamp(u64);

impl Timestamp {
    pub const MIN: Self = Self(u64::MIN);
    pub const MAX: Self = Self(u64::MAX);

    #[must_use]
    pub fn now() -> Self {
        Self(icu::utils::time::now_secs())
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl FieldValue for Timestamp {
    fn to_value(&self) -> Value {
        Value::Timestamp(*self)
    }
}

impl From<u64> for Timestamp {
    fn from(u: u64) -> Self {
        Self(u)
    }
}

impl Sanitize for Timestamp {}

impl TypeView for Timestamp {
    type View = Self;

    fn to_view(&self) -> Self::View {
        *self
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

impl ValidateAuto for Timestamp {}

impl ValidateCustom for Timestamp {}

impl Visitable for Timestamp {}
