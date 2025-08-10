use crate::core::{
    Value,
    traits::{FieldValue, TypeView, ValidateAuto, ValidateCustom, Visitable},
};
use candid::{CandidType, Nat as WrappedNat};
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, FromStr, Sub, SubAssign};
use icu::impl_storable_unbounded;
use serde::{Deserialize, Serialize};

///
/// Nat
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
pub struct Nat(WrappedNat);

impl Nat {
    #[must_use]
    pub fn to_leb128(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.encode(&mut out).expect("Nat LEB128 encode");

        out
    }
}

impl FieldValue for Nat {
    fn to_value(&self) -> Value {
        Value::BigUint(self.clone())
    }
}

impl From<WrappedNat> for Nat {
    fn from(n: WrappedNat) -> Self {
        Self(n)
    }
}

impl_storable_unbounded!(Nat);

impl TypeView for Nat {
    type View = Self;

    fn to_view(&self) -> Self::View {
        self.clone()
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

impl ValidateAuto for Nat {}

impl ValidateCustom for Nat {}

impl Visitable for Nat {}
