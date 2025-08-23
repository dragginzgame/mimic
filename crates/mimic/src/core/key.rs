use crate::core::{
    Value,
    traits::FieldValue,
    types::{Principal, Subaccount, Timestamp, Ulid},
};
use candid::{CandidType, Principal as WrappedPrincipal};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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
    Principal(Principal),
    Subaccount(Subaccount),
    Timestamp(Timestamp),
    Uint(u64),
    Ulid(Ulid),
}

impl Key {
    pub const MIN: Self = Self::Int(i64::MIN); // global minimum
    pub const MAX: Self = Self::Ulid(Ulid::MAX); // global maximum

    #[must_use]
    pub const fn max_storable() -> Self {
        Self::Principal(Principal::max_storable())
    }

    const fn variant_rank(&self) -> u8 {
        match self {
            Self::Int(_) => 0,
            Self::Principal(_) => 1,
            Self::Subaccount(_) => 2,
            Self::Timestamp(_) => 3,
            Self::Uint(_) => 4,
            Self::Ulid(_) => 5,
        }
    }
}

impl FieldValue for Key {
    fn to_value(&self) -> Value {
        match self {
            Self::Int(v) => Value::Int(*v),
            Self::Uint(v) => Value::Uint(*v),
            Self::Principal(v) => Value::Principal(*v),
            Self::Subaccount(v) => Value::Subaccount(*v),
            Self::Timestamp(v) => Value::Timestamp(*v),
            Self::Ulid(v) => Value::Ulid(*v),
        }
    }
}

/// Implements `From<T> for Key` for simple conversions
macro_rules! impl_from_key {
    ( $( $ty:ty => $variant:ident ),* $(,)? ) => {
        $(
            impl From<$ty> for Key {
                fn from(v: $ty) -> Self {
                    Self::$variant(v.into())
                }
            }
        )*
    }
}

/// Implements symmetric PartialEq between Key and another type
macro_rules! impl_eq_key {
    ( $( $ty:ty => $variant:ident ),* $(,)? ) => {
        $(
            impl PartialEq<$ty> for Key {
                fn eq(&self, other: &$ty) -> bool {
                    matches!(self, Self::$variant(val) if val == other)
                }
            }

            impl PartialEq<Key> for $ty {
                fn eq(&self, other: &Key) -> bool {
                    other == self
                }
            }
        )*
    }
}

impl_from_key! {
    i8  => Int,
    i16 => Int,
    i32 => Int,
    i64 => Int,
    u8  => Uint,
    u16 => Uint,
    u32 => Uint,
    u64 => Uint,
    Ulid => Ulid,
    Principal => Principal,
    WrappedPrincipal => Principal,
    Subaccount => Subaccount,
}

impl_eq_key! {
    i64 => Int,
    u64  => Uint,
    Ulid => Ulid,
    Principal => Principal,
    Subaccount => Subaccount,
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => Ord::cmp(a, b),
            (Self::Principal(a), Self::Principal(b)) => Ord::cmp(a, b),
            (Self::Uint(a), Self::Uint(b)) => Ord::cmp(a, b),
            (Self::Ulid(a), Self::Ulid(b)) => Ord::cmp(a, b),

            _ => Ord::cmp(&self.variant_rank(), &other.variant_rank()), // fallback for cross-type comparison
        }
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
