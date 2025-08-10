use crate::core::traits::{FieldValue, TypeView, ValidateAuto, ValidateCustom, Visitable};
use candid::{CandidType, Int as WrappedInt};
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, FromStr, Sub, SubAssign};
use icu::impl_storable_unbounded;
use serde::{Deserialize, Serialize};

///
/// Int
///

#[derive(
    Add,
    AddAssign,
    CandidType,
    Clone,
    Debug,
    Default,
    Deref,
    DerefMut,
    Display,
    Eq,
    PartialEq,
    FromStr,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    Sub,
    SubAssign,
)]
pub struct Int(WrappedInt);

impl FieldValue for Int {}

impl From<WrappedInt> for Int {
    fn from(i: WrappedInt) -> Self {
        Self(i)
    }
}

impl_storable_unbounded!(Int);

impl TypeView for Int {
    type View = Self;

    fn to_view(&self) -> Self::View {
        self.clone()
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

impl ValidateAuto for Int {}

impl ValidateCustom for Int {}

impl Visitable for Int {}
