pub mod fixture;
pub mod generator;

use candid::CandidType;
use derive_more::{Deref, DerefMut, FromStr};
use ic::structures::{
    serialize::{from_binary, to_binary},
    storable::Bound,
    Storable,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use snafu::Snafu;
use std::{borrow::Cow, fmt};
use ulid::Ulid as WrappedUlid;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("ulid is nil"))]
    Nil,

    #[snafu(display("invalid character found"))]
    InvalidChar,

    #[snafu(display("ulid has an invalid length"))]
    InvalidLength,
}

impl From<ulid::DecodeError> for Error {
    fn from(error: ulid::DecodeError) -> Self {
        match error {
            ulid::DecodeError::InvalidChar => Self::InvalidChar,
            ulid::DecodeError::InvalidLength => Self::InvalidLength,
        }
    }
}

///
/// Ulid
/// a wrapper around ulid::Ulid
///

#[derive(Clone, Copy, Debug, Deref, DerefMut, Eq, FromStr, PartialEq, Hash, Ord, PartialOrd)]
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

    /// from_string
    pub fn from_string(encoded: &str) -> Result<Self, Error> {
        let ulid = WrappedUlid::from_string(encoded)?;

        Ok(Self(ulid))
    }

    /// generate
    /// Generate a ULID string with the current timestamp and a random value
    #[must_use]
    pub fn generate() -> Self {
        generator::generate().unwrap()
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

impl Storable for Ulid {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(to_binary(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        from_binary(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
