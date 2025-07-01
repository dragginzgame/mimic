use crate::core::traits::{
    FieldSearchable, FieldSortable, FieldValue, Inner, ValidateAuto, ValidateCustom, Visitable,
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

impl FieldSearchable for Int {
    fn to_searchable_string(&self) -> Option<String> {
        Some(self.to_string())
    }
}

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

impl Inner for Int {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        self.clone()
    }

    fn into_inner(self) -> Self::Primitive {
        self
    }
}

impl_storable_unbounded!(Int);

impl ValidateAuto for Int {}

impl ValidateCustom for Int {}

impl Visitable for Int {}
