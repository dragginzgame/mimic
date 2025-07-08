use crate::core::{
    db::EntityKey,
    types::{Decimal, E8s, E18s, Principal, Ulid},
};
use candid::{CandidType, Principal as WrappedPrincipal};
use derive_more::{Deref, DerefMut, Display};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap};
use thiserror::Error as ThisError;

///
/// ValueError
///

#[derive(Debug, ThisError)]
pub enum ValueError {
    #[error("index value conversion fail")]
    IndexValueConversion,
}

///
/// Handy Macros
///

macro_rules! impl_from_for {
    ( $struct:ty, $( $type:ty => $variant:ident ),* $(,)? ) => {
        $(
            impl From<$type> for $struct {
                fn from(v: $type) -> Self {
                    Self::$variant(v.into())
                }
            }
        )*
    };
}

///
/// Values
/// a HashMap of Values returned from the Entity
///

#[derive(Debug, Deref, DerefMut)]
pub struct Values(pub HashMap<&'static str, Value>);

impl Values {
    #[must_use]
    pub fn collect_all(&self, fields: &[&str]) -> Vec<Value> {
        let mut values = Vec::with_capacity(fields.len());

        for field in fields {
            if let Some(v) = self.0.get(field) {
                values.push(v.clone());
            }
        }

        values
    }
}

///
/// Value
/// can be searched or used in WHERE statements
///

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Value {
    Bool(bool),
    Decimal(Decimal),
    E8s(E8s),
    E18s(E18s),
    Float(f64),
    Int(i64),
    Nat(u64),
    Principal(Principal),
    Text(String),
    Ulid(Ulid),
    Null,
    Unsupported,
}

impl Value {
    #[must_use]
    pub fn into_index_value(self) -> Option<IndexValue> {
        match self {
            Self::Int(v) => Some(IndexValue::Int(v)),
            Self::Nat(v) => Some(IndexValue::Nat(v)),
            Self::Principal(v) => Some(IndexValue::Principal(v)),
            Self::Ulid(v) => Some(IndexValue::Ulid(v)),
            _ => None,
        }
    }

    /// Return the unmodified searchable string
    #[must_use]
    pub fn to_searchable_string(&self) -> Option<String> {
        match self {
            Self::Decimal(v) => Some(v.to_string()),
            Self::Principal(v) => Some(v.to_text()),
            Self::Text(v) => Some(v.to_string()),
            Self::Ulid(v) => Some(v.to_string()),
            _ => None,
        }
    }
}

impl_from_for! {
    Value,
    bool => Bool,
    Decimal => Decimal,
    E8s => E8s,
    E18s => E18s,
    f32 => Float,
    f64 => Float,
    i8 => Int,
    i16 => Int,
    i32 => Int,
    i64 => Int,
    Principal => Principal,
    &str => Text,
    String => Text,
    Ulid => Ulid,
    u8 => Nat,
    u16 => Nat,
    u32 => Nat,
    u64 => Nat,
}

impl From<EntityKey> for Value {
    fn from(value: EntityKey) -> Self {
        value[0].into()
    }
}

impl From<IndexValue> for Value {
    fn from(iv: IndexValue) -> Self {
        match iv {
            IndexValue::Int(v) => Self::Int(v),
            IndexValue::Nat(v) => Self::Nat(v),
            IndexValue::Principal(v) => Self::Principal(v),
            IndexValue::Ulid(v) => Self::Ulid(v),
        }
    }
}

impl From<WrappedPrincipal> for Value {
    fn from(v: WrappedPrincipal) -> Self {
        Self::Principal(v.into())
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a.partial_cmp(b),
            (Self::E8s(a), Self::E8s(b)) => a.partial_cmp(b),
            (Self::E18s(a), Self::E18s(b)) => a.partial_cmp(b),
            (Self::Float(a), Self::Float(b)) => a.partial_cmp(b),
            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            (Self::Nat(a), Self::Nat(b)) => a.partial_cmp(b),
            (Self::Principal(a), Self::Principal(b)) => a.partial_cmp(b),
            (Self::Text(a), Self::Text(b)) => a.partial_cmp(b),
            (Self::Ulid(a), Self::Ulid(b)) => a.partial_cmp(b),

            // Cross-type comparisons: no ordering
            _ => None,
        }
    }
}

///
/// IndexValue
///
/// Treating IndexValue as the atomic, normalized unit of the keyspace
/// Backing primary keys and secondary indexes with the same value representation
/// Planning to enforce Copy semantics (i.e., fast, clean, safe)
///

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub enum IndexValue {
    Int(i64),
    Nat(u64),
    Principal(Principal),
    Ulid(Ulid),
}

impl IndexValue {
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

impl_from_for! {
    IndexValue,
    i8 => Int,
    i16 => Int,
    i32 => Int,
    i64 => Int,
    Principal => Principal,
    Ulid => Ulid,
    u8 => Nat,
    u16 => Nat,
    u32 => Nat,
    u64 => Nat,
}

impl From<EntityKey> for IndexValue {
    fn from(value: EntityKey) -> Self {
        value[0]
    }
}

impl From<WrappedPrincipal> for IndexValue {
    fn from(p: WrappedPrincipal) -> Self {
        Self::Principal(p.into())
    }
}

impl From<[Self; 1]> for IndexValue {
    fn from(value: [Self; 1]) -> Self {
        value[0]
    }
}

impl Ord for IndexValue {
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

impl PartialOrd for IndexValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<IndexValue> for u64 {
    type Error = ValueError;

    fn try_from(value: IndexValue) -> Result<Self, Self::Error> {
        match value {
            IndexValue::Nat(n) => Ok(n),
            _ => Err(ValueError::IndexValueConversion),
        }
    }
}

impl TryFrom<IndexValue> for i64 {
    type Error = ValueError;

    fn try_from(value: IndexValue) -> Result<Self, Self::Error> {
        match value {
            IndexValue::Int(i) => Ok(i),
            _ => Err(ValueError::IndexValueConversion),
        }
    }
}

impl TryFrom<IndexValue> for Principal {
    type Error = ValueError;

    fn try_from(value: IndexValue) -> Result<Self, Self::Error> {
        match value {
            IndexValue::Principal(p) => Ok(p),
            _ => Err(ValueError::IndexValueConversion),
        }
    }
}

impl TryFrom<IndexValue> for Ulid {
    type Error = ValueError;

    fn try_from(value: IndexValue) -> Result<Self, Self::Error> {
        match value {
            IndexValue::Ulid(id) => Ok(id),
            _ => Err(ValueError::IndexValueConversion),
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
    fn ulid_max_is_highest_index_value() {
        let max = IndexValue::MAX;

        let others = vec![
            IndexValue::Int(i64::MAX),
            IndexValue::Nat(u64::MAX),
            IndexValue::Principal(Principal::from_slice(&[0xFF; 29])),
            IndexValue::Ulid(Ulid::from_u128(0)),
        ];

        for v in others {
            assert!(v < max, "Expected {v:?} < Ulid::MAX");
        }
    }
}
