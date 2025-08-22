use crate::core::{
    Value,
    traits::{FieldValue, TypeView, ValidateAuto, ValidateCustom, Visitable},
};
use candid::CandidType;
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, FromStr, Sub, SubAssign};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

///
/// Nat128
///

#[derive(
    Add,
    AddAssign,
    CandidType,
    Clone,
    Copy,
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
    Sub,
    SubAssign,
)]
pub struct Nat128(u128);

impl Nat128 {
    #[must_use]
    pub const fn get(self) -> u128 {
        self.0
    }
}

impl FieldValue for Nat128 {
    fn to_value(&self) -> Value {
        Value::Uint128(*self)
    }
}

impl From<u128> for Nat128 {
    fn from(u: u128) -> Self {
        Self(u)
    }
}

impl PartialEq<u128> for Nat128 {
    fn eq(&self, other: &u128) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Nat128> for u128 {
    fn eq(&self, other: &Nat128) -> bool {
        *self == other.0
    }
}

impl PartialOrd<u128> for Nat128 {
    fn partial_cmp(&self, other: &u128) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Nat128> for u128 {
    fn partial_cmp(&self, other: &Nat128) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl Serialize for Nat128 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_be_bytes())
    }
}

impl<'de> Deserialize<'de> for Nat128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: &[u8] = serde::Deserialize::deserialize(deserializer)?;
        if bytes.len() == 16 {
            let mut arr = [0u8; 16];
            arr.copy_from_slice(bytes);

            Ok(Self(u128::from_be_bytes(arr)))
        } else {
            Err(serde::de::Error::custom("expected 16 bytes"))
        }
    }
}

impl TypeView for Nat128 {
    type View = Self;

    fn to_view(&self) -> Self::View {
        *self
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

impl ValidateAuto for Nat128 {}

impl ValidateCustom for Nat128 {}

impl Visitable for Nat128 {}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{deserialize, serialize};

    fn roundtrip(v: u128) {
        let nat128 = Nat128::from(v);

        // serialize
        let bytes = serialize(&nat128).expect("serialize failed");

        // must be length-prefixed
        // so length = 16 + 1/2 bytes overhead, but we just check round-trip.
        let decoded: Nat128 = deserialize(&bytes).expect("deserialize failed");

        assert_eq!(decoded, nat128, "roundtrip failed for {v}");

        // sanity check on raw serialization: inner payload must be 16 bytes
        let raw = nat128.0.to_be_bytes();
        let encoded_inner = &bytes[bytes.len() - 16..];
        assert_eq!(encoded_inner, &raw, "encoded inner bytes mismatch");
    }

    #[test]
    fn test_roundtrip_basic() {
        roundtrip(1);
        roundtrip(1_234_567_890_123_456_789);
    }

    #[test]
    fn test_roundtrip_edges() {
        roundtrip(u128::MIN);
        roundtrip(u128::MAX);
    }

    #[test]
    fn test_manual_encoding() {
        let v = Nat128::from(42);
        let bytes = serialize(&v).unwrap();
        let encoded_inner = &bytes[bytes.len() - 16..];
        assert_eq!(encoded_inner, &42i128.to_be_bytes());
    }
}
