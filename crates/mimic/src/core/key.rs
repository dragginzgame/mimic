use crate::core::types::{Principal, Ulid};
use candid::{CandidType, Principal as WrappedPrincipal};
use derive_more::{Deref, DerefMut, Display};
use icu::impl_storable_bounded;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use thiserror::Error as ThisError;

///
/// KeyError
///

#[derive(Debug, ThisError)]
pub enum KeyError {
    #[error("key conversion failed")]
    KeyConversion,
}

///
/// Keys
///

#[derive(
    CandidType,
    Clone,
    Debug,
    Default,
    Deref,
    DerefMut,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct Keys(pub Vec<Key>);

impl Keys {
    pub fn iter(&self) -> impl Iterator<Item = &Key> {
        self.0.iter()
    }
}

///
/// Key
///
/// Treating IndexKey as the atomic, normalized unit of the keyspace
/// Backing primary keys and secondary indexes with the same value representation
/// Planning to enforce Copy semantics (i.e., fast, clean, safe)
///

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub enum Key {
    Int(i64),
    Nat(u64),
    Principal(Principal),
    Ulid(Ulid),
}

impl Key {
    pub const MIN: Self = Self::Int(i64::MIN);
    pub const MAX: Self = Self::Ulid(Ulid::MAX);

    const fn variant_rank(&self) -> u8 {
        match self {
            Self::Int(_) => 0,
            Self::Nat(_) => 1,
            Self::Principal(_) => 2,
            Self::Ulid(_) => 3,
        }
    }

    /// Returns the maximum possible index value for range upper bounds.
    #[must_use]
    pub const fn sentinel_max(&self) -> Self {
        match self {
            Self::Int(_) => Self::Int(i64::MAX),
            Self::Nat(_) => Self::Nat(u64::MAX),
            Self::Principal(_) => Self::Principal(Principal::MAX),
            Self::Ulid(_) => Self::Ulid(Ulid::MAX),
        }
    }
}

impl From<i32> for Key {
    fn from(v: i32) -> Self {
        Self::Int(v.into())
    }
}

impl From<u64> for Key {
    fn from(v: u64) -> Self {
        Self::Nat(v)
    }
}

impl From<Ulid> for Key {
    fn from(v: Ulid) -> Self {
        Self::Ulid(v)
    }
}

impl From<Principal> for Key {
    fn from(p: Principal) -> Self {
        Self::Principal(p)
    }
}

impl From<WrappedPrincipal> for Key {
    fn from(p: WrappedPrincipal) -> Self {
        Self::Principal(p.into())
    }
}

impl PartialEq<i64> for Key {
    fn eq(&self, other: &i64) -> bool {
        matches!(self, Key::Int(val) if val == other)
    }
}

impl PartialEq<Key> for i64 {
    fn eq(&self, other: &Key) -> bool {
        other == self
    }
}

impl PartialEq<u64> for Key {
    fn eq(&self, other: &u64) -> bool {
        matches!(self, Key::Nat(val) if val == other)
    }
}

impl PartialEq<Key> for u64 {
    fn eq(&self, other: &Key) -> bool {
        other == self
    }
}

impl PartialEq<Ulid> for Key {
    fn eq(&self, other: &Ulid) -> bool {
        matches!(self, Key::Ulid(val) if val == other)
    }
}

impl PartialEq<Key> for Ulid {
    fn eq(&self, other: &Key) -> bool {
        other == self
    }
}

impl PartialEq<Principal> for Key {
    fn eq(&self, other: &Principal) -> bool {
        matches!(self, Key::Principal(val) if val == other)
    }
}

impl PartialEq<Key> for Principal {
    fn eq(&self, other: &Key) -> bool {
        other == self
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::Nat(a), Self::Nat(b)) => a.cmp(b),
            (Self::Principal(a), Self::Principal(b)) => a.cmp(b),
            (Self::Ulid(a), Self::Ulid(b)) => a.cmp(b),

            _ => self.variant_rank().cmp(&other.variant_rank()), // fallback for cross-type comparison
        }
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl_storable_bounded!(Key, 30, false);

impl TryFrom<Key> for u64 {
    type Error = KeyError;

    fn try_from(value: Key) -> Result<Self, Self::Error> {
        match value {
            Key::Nat(n) => Ok(n),
            _ => Err(KeyError::KeyConversion),
        }
    }
}

impl TryFrom<Key> for i64 {
    type Error = KeyError;

    fn try_from(value: Key) -> Result<Self, Self::Error> {
        match value {
            Key::Int(i) => Ok(i),
            _ => Err(KeyError::KeyConversion),
        }
    }
}

impl TryFrom<Key> for Principal {
    type Error = KeyError;

    fn try_from(value: Key) -> Result<Self, Self::Error> {
        match value {
            Key::Principal(p) => Ok(p),
            _ => Err(KeyError::KeyConversion),
        }
    }
}

impl TryFrom<Key> for Ulid {
    type Error = KeyError;

    fn try_from(value: Key) -> Result<Self, Self::Error> {
        match value {
            Key::Ulid(id) => Ok(id),
            _ => Err(KeyError::KeyConversion),
        }
    }
}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_is_32_bytes() {
        assert_eq!(std::mem::size_of::<Key>(), 32);
    }

    #[test]
    fn ulid_min_is_lowest_key() {
        let min = Key::MIN;

        let others = vec![
            Key::Ulid(Ulid::MIN),
            Key::Nat(u64::MIN),
            Key::Principal(Principal::MIN),
        ];

        for v in others {
            assert!(v > min, "Expected {v:?} > Key::MIN");
        }
    }

    #[test]
    fn ulid_max_is_highest_key() {
        let max = Key::MAX;

        let others = vec![
            Key::Int(i64::MAX),
            Key::Nat(u64::MAX),
            Key::Principal(Principal::MAX),
        ];

        for v in others {
            assert!(v < max, "Expected {v:?} < Key::MAX");
        }
    }
}
