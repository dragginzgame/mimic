use crate::core::traits::{
    FieldSortable, FieldValue, TypeView, ValidateAuto, ValidateCustom, Visitable,
};
use candid::{CandidType, Int as WrappedInt};
use derive_more::{Deref, DerefMut, Display, FromStr};
use icu::impl_storable_unbounded;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

///
/// Int
///

#[derive(
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
)]
pub struct Int(WrappedInt);

impl FieldSortable for Int {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldValue for Int {}

impl From<WrappedInt> for Int {
    fn from(i: WrappedInt) -> Self {
        Self(i)
    }
}

impl_storable_unbounded!(Int);

impl TypeView for Int {
    type View = WrappedInt;

    fn to_view(&self) -> Self::View {
        self.0.clone()
    }

    fn from_view(view: Self::View) -> Self {
        Self(view)
    }
}

impl ValidateAuto for Int {}

impl ValidateCustom for Int {}

impl Visitable for Int {}
