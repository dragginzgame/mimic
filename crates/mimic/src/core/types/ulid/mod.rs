pub mod fixture;
pub mod generator;

use crate::{
    ThisError,
    core::{
        db::EntityKey,
        traits::{
            FieldSearchable, FieldSortable, FieldValue, Inner, TypeView, ValidateAuto,
            ValidateCustom, Visitable,
        },
        value::{IndexValue, Value},
    },
};
use ::ulid::Ulid as WrappedUlid;
use candid::CandidType;
use derive_more::{Deref, DerefMut, Display, FromStr};
use icu::impl_storable_bounded;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;

///
/// Error
///

#[derive(Debug, ThisError)]
pub enum UlidError {
    #[error("invalid character found")]
    InvalidChar,

    #[error("ulid has an invalid length")]
    InvalidLength,

    #[error("invalid ulid string")]
    InvalidString,

    #[error("monotonic error - overflow")]
    GeneratorOverflow,
}

impl From<::ulid::DecodeError> for UlidError {
    fn from(error: ::ulid::DecodeError) -> Self {
        match error {
            ::ulid::DecodeError::InvalidChar => Self::InvalidChar,
            ::ulid::DecodeError::InvalidLength => Self::InvalidLength,
        }
    }
}

///
/// Ulid
///

#[derive(
    Clone, Copy, Debug, Deref, DerefMut, Display, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct Ulid(WrappedUlid);

impl Ulid {
    pub const MAX: Self = Self::from_bytes([0xFF; 16]);

    /// nil
    #[must_use]
    pub const fn nil() -> Self {
        Self(WrappedUlid::nil())
    }

    /// from_parts
    #[must_use]
    pub const fn from_parts(timestamp_ms: u64, random: u128) -> Self {
        Self(WrappedUlid::from_parts(timestamp_ms, random))
    }

    /// generate
    /// Generate a ULID string with the current timestamp and a random value
    #[must_use]
    pub fn generate() -> Self {
        generator::generate().unwrap()
    }

    /// from_bytes
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(WrappedUlid::from_bytes(bytes))
    }

    /// from_str
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(encoded: &str) -> Result<Self, UlidError> {
        let this = WrappedUlid::from_string(encoded).map_err(|_| UlidError::InvalidString)?;

        Ok(Self(this))
    }

    /// from_u128
    #[must_use]
    pub const fn from_u128(n: u128) -> Self {
        Self(WrappedUlid::from_bytes(n.to_be_bytes()))
    }
}

impl CandidType for Ulid {
    fn _ty() -> candid::types::Type {
        candid::types::TypeInner::Text.into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        serializer.serialize_text(&self.0.to_string())
    }
}

impl Default for Ulid {
    fn default() -> Self {
        Self(WrappedUlid::nil())
    }
}

impl FieldSearchable for Ulid {
    fn to_searchable_string(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl FieldSortable for Ulid {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldValue for Ulid {
    fn to_value(&self) -> Value {
        Value::Ulid(*self)
    }
}

impl Inner for Ulid {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        *self
    }

    fn into_inner(self) -> Self::Primitive {
        self
    }
}

impl<T: Into<WrappedUlid>> From<T> for Ulid {
    fn from(t: T) -> Self {
        Self(t.into())
    }
}

// Serialize and Deserialize from the ulid crate just don't compile
impl Serialize for Ulid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buffer = [0; ::ulid::ULID_LEN];
        let text = self.array_to_str(&mut buffer);
        text.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Ulid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized_str = String::deserialize(deserializer)?;
        let ulid = WrappedUlid::from_string(&deserialized_str).map_err(serde::de::Error::custom)?;

        Ok(Self(ulid))
    }
}

impl_storable_bounded!(Ulid, 16, true);

impl TryFrom<EntityKey> for Ulid {
    type Error = &'static str;

    fn try_from(key: EntityKey) -> Result<Self, Self::Error> {
        match key.as_slice() {
            [IndexValue::Ulid(id)] => Ok(*id),
            _ => Err("Expected single Ulid in EntityKey"),
        }
    }
}

impl TryFrom<&EntityKey> for Ulid {
    type Error = &'static str;

    fn try_from(key: &EntityKey) -> Result<Self, Self::Error> {
        match key.as_slice() {
            [IndexValue::Ulid(id)] => Ok(*id),
            _ => Err("Expected single Ulid in EntityKey"),
        }
    }
}

impl TypeView for Ulid {
    type View = Self;

    fn to_view(&self) -> Self::View {
        *self
    }

    fn from_view(view: Self::View) -> Self {
        Self(*view)
    }
}

impl ValidateAuto for Ulid {}

impl ValidateCustom for Ulid {}

impl Visitable for Ulid {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ulid_string_roundtrip() {
        let u1 = Ulid::generate();
        let u2 = Ulid::from_str(&u1.to_string()).unwrap();

        assert_eq!(u1, u2);
    }
}
