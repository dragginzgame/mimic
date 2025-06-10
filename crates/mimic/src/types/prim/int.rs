use crate::{
    prelude::*,
    traits::{FormatSortKey, Inner, ValidateAuto},
};
use candid::{CandidType, Int as WrappedInt};
use derive_more::{Deref, DerefMut, FromStr};
use icu::impl_storable_unbounded;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{self},
};

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

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

// Int shouldn't be used as a SortKey as its unbounded
impl FormatSortKey for Int {
    fn format_sort_key(&self) -> Option<String> {
        None
    }
}

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

impl Orderable for Int {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl Searchable for Int {
    fn as_text(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl_storable_unbounded!(Int);

impl ValidateAuto for Int {}

impl ValidateCustom for Int {}

impl Visitable for Int {}
