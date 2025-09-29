use crate::core::{
    Value,
    traits::{
        FieldValue, SanitizeAuto, SanitizeCustom, TypeView, ValidateAuto, ValidateCustom, Visitable,
    },
};
use candid::CandidType;
use derive_more::{Add, AddAssign, Display, FromStr, Sub, SubAssign};
use serde::{Deserialize, Serialize};

///
/// Duration
/// (in milliseconds)
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
pub struct Duration(u64);

impl Duration {
    pub const ZERO: Self = Self(0);
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

impl FieldValue for Duration {
    fn to_value(&self) -> Value {
        Value::Duration(*self)
    }
}

impl From<u64> for Duration {
    fn from(u: u64) -> Self {
        Self(u)
    }
}

impl SanitizeAuto for Duration {}

impl SanitizeCustom for Duration {}

impl TypeView for Duration {
    type View = Self;

    fn to_view(&self) -> Self::View {
        *self
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

impl ValidateAuto for Duration {}

impl ValidateCustom for Duration {}

impl Visitable for Duration {}
