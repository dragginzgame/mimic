use crate::{
    prelude::*,
    traits::{Inner, ValidateAuto},
};
use candid::{CandidType, Nat as WrappedNat};
use derive_more::{Deref, DerefMut};
use icu::impl_storable_unbounded;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt};

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
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct Nat(WrappedNat);

impl fmt::Display for Nat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<WrappedNat> for Nat {
    fn from(n: WrappedNat) -> Self {
        Self(n)
    }
}

impl Inner<Self> for Nat {
    fn inner(&self) -> &Self {
        self
    }

    fn into_inner(self) -> Self {
        self
    }
}

impl Orderable for Nat {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl Path for Nat {
    const IDENT: &'static str = "Nat";
    const PATH: &'static str = "mimic::types::prim::Nat";
}

impl Searchable for Nat {
    fn as_text(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl_storable_unbounded!(Nat);

impl ValidateAuto for Nat {}

impl ValidateCustom for Nat {}

impl Visitable for Nat {}
