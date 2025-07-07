use crate::core::traits::{
    FieldSearchable, FieldSortable, FieldValue, Inner, TypeView, ValidateAuto, ValidateCustom,
    Visitable,
};
use candid::{CandidType, Nat as WrappedNat};
use derive_more::{Deref, DerefMut, Display, FromStr};
use icu::impl_storable_unbounded;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

///
/// Nat
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
pub struct Nat(WrappedNat);

impl FieldSearchable for Nat {
    fn to_searchable_string(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl FieldSortable for Nat {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldValue for Nat {}

impl From<WrappedNat> for Nat {
    fn from(n: WrappedNat) -> Self {
        Self(n)
    }
}

impl Inner for Nat {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        self.clone()
    }

    fn into_inner(self) -> Self::Primitive {
        self
    }
}

impl_storable_unbounded!(Nat);

impl TypeView for Nat {
    type View = WrappedNat;

    fn to_view(&self) -> Self::View {
        self.0.clone()
    }

    fn from_view(view: Self::View) -> Self {
        Self(view)
    }
}

impl ValidateAuto for Nat {}

impl ValidateCustom for Nat {}

impl Visitable for Nat {}
