pub mod fixture;
pub mod generator;

use crate::{
    Error, ThisError,
    ic::serialize::{deserialize, serialize},
    impl_storable_bounded,
    orm::{
        prelude::*,
        traits::{EntityId, Filterable, Inner, Orderable, SortKey, ValidateAuto},
    },
    types::ErrorTree,
};
use derive_more::{Deref, DerefMut, From, FromStr, IntoIterator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{borrow::Cow, cmp::Ordering, fmt};
use ulid::Ulid as WrappedUlid;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum UlidError {
    #[error("ulid is nil")]
    Nil,

    #[error("invalid character found")]
    InvalidChar,

    #[error("ulid has an invalid length")]
    InvalidLength,

    #[error("invalid ulid string")]
    InvalidString,

    #[error("monotonic error - overflow")]
    GeneratorOverflow,
}

impl From<ulid::DecodeError> for UlidError {
    fn from(error: ulid::DecodeError) -> Self {
        match error {
            ulid::DecodeError::InvalidChar => Self::InvalidChar,
            ulid::DecodeError::InvalidLength => Self::InvalidLength,
        }
    }
}

///
/// Ulid
///

#[derive(Clone, Copy, Debug, Deref, DerefMut, Eq, PartialEq, FromStr, Hash, Ord, PartialOrd)]
pub struct Ulid(WrappedUlid);

impl Ulid {
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

    /// from_str
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(encoded: &str) -> Result<Self, UlidError> {
        let this = WrappedUlid::from_string(encoded).map_err(|_| UlidError::InvalidString)?;

        Ok(Self(this))
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

impl fmt::Display for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Filterable for Ulid {
    fn as_text(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Inner<Self> for Ulid {
    fn inner(&self) -> &Self {
        self
    }

    fn into_inner(self) -> Self {
        self
    }
}

impl<T: Into<WrappedUlid>> From<T> for Ulid {
    fn from(t: T) -> Self {
        Self(t.into())
    }
}

impl Orderable for Ulid {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

// Serialize and Deserialize from the ulid crate just don't compile
impl Serialize for Ulid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buffer = [0; ulid::ULID_LEN];
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
        let ulid = WrappedUlid::from_string(&deserialized_str).unwrap_or_default();

        Ok(Self(ulid))
    }
}

impl SortKey for Ulid {}

impl_storable_bounded!(Ulid, 16, true);

impl ValidateManual for Ulid {
    fn validate_manual(&self) -> Result<(), ErrorTree> {
        if self.is_nil() {
            Err(UlidError::Nil.to_string().into())
        } else {
            Ok(())
        }
    }
}

impl ValidateAuto for Ulid {}

impl Visitable for Ulid {}

///
/// UlidSet
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
    From,
    IntoIterator,
    Serialize,
    Deserialize,
)]
pub struct UlidSet(HashSet<Ulid>);

impl fmt::Display for UlidSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self
            .0
            .iter()
            .map(Ulid::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{formatted}]")
    }
}

impl<T: EntityId> From<Vec<T>> for UlidSet {
    fn from(ids: Vec<T>) -> Self {
        Self(ids.into_iter().map(|id| id.ulid()).collect())
    }
}

/// Allow `UlidSet` to be iterated over by reference (`for ulid in &ulid_set`)
impl<'a> IntoIterator for &'a UlidSet {
    type Item = &'a Ulid;
    type IntoIter = std::collections::hash_set::Iter<'a, Ulid>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
