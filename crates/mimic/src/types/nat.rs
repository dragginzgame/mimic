use crate::{
    ops::traits::{FieldOrderable, FieldQueryable, Inner, ValidateAuto, Visitable},
    prelude::*,
};
use candid::{CandidType, Nat as WrappedNat};
use derive_more::{Deref, DerefMut, FromStr};
use icu::impl_storable_unbounded;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

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

impl Display for Nat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FieldOrderable for Nat {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldQueryable for Nat {
    fn to_query_value(&self) -> Option<String> {
        Some(self.to_string())
    }
}

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

impl ValidateAuto for Nat {}

impl ValidateCustom for Nat {}

impl Visitable for Nat {}
