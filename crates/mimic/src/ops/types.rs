use crate::types::{Blob, Decimal, EntityKey, Principal, Ulid, Unit};
use candid::{CandidType, Principal as WrappedPrincipal};
use derive_more::{Deref, DerefMut, Display};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap};

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

macro_rules! impl_from_ref_for {
    ( $struct:ty, $( $type:ty => $variant:ident ),* $(,)? ) => {
        $(
            impl From<&$type> for $struct {
                fn from(v: &$type) -> Self {
                    Self::$variant(v.clone())
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
                values.push(v.clone())
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
    Blob, // don't want the data right now
    Decimal(Decimal),
    EntityKey(EntityKey),
    Float(f64),
    Int(i64),
    Nat(u64),
    Principal(Principal),
    Text(String),
    Ulid(Ulid),
    Unit,
    Unsupported,
}

impl Value {
    #[must_use]
    pub fn into_index_value(self) -> Option<IndexValue> {
        match self {
            Value::Decimal(d) => Some(IndexValue::Decimal(d)),
            Value::EntityKey(k) => Some(IndexValue::EntityKey(k)),
            Value::Int(i) => Some(IndexValue::Int(i)),
            Value::Nat(n) => Some(IndexValue::Nat(n)),
            Value::Principal(p) => Some(IndexValue::Principal(p)),
            Value::Text(s) => Some(IndexValue::Text(s)),
            Value::Ulid(u) => Some(IndexValue::Ulid(u)),
            _ => None,
        }
    }
}

impl_from_for! {
    Value,
    bool => Bool,
    Decimal => Decimal,
    EntityKey => EntityKey,
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

impl_from_ref_for! {
    Value,
    EntityKey => EntityKey,
}

impl From<Blob> for Value {
    fn from(_: Blob) -> Self {
        Self::Blob
    }
}

impl From<Unit> for Value {
    fn from(_: Unit) -> Self {
        Self::Unit
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
            (Self::Decimal(a), Self::Decimal(b)) => a.partial_cmp(b),
            (Self::EntityKey(a), Self::EntityKey(b)) => a.partial_cmp(b),
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
/// strictly for indexable fields (DataKey, EntityKey)
///

#[derive(CandidType, Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub enum IndexValue {
    Decimal(Decimal),
    EntityKey(EntityKey),
    Int(i64),
    Nat(u64),
    Principal(Principal),
    Text(String),
    Ulid(Ulid),
    UpperBoundMarker,
}

impl IndexValue {
    const fn variant_rank(&self) -> u8 {
        match self {
            Self::Decimal(_) => 0,
            Self::EntityKey(_) => 1,
            Self::Int(_) => 2,
            Self::Nat(_) => 3,
            Self::Principal(_) => 4,
            Self::Text(_) => 5,
            Self::Ulid(_) => 6,
            Self::UpperBoundMarker => u8::MAX,
        }
    }
}

impl_from_for! {
    IndexValue,
    Decimal => Decimal,
    EntityKey => EntityKey,
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

impl_from_ref_for! {
    IndexValue,
    EntityKey => EntityKey,
}

impl From<candid::Principal> for IndexValue {
    fn from(p: candid::Principal) -> Self {
        Self::Principal(p.into())
    }
}

impl Ord for IndexValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::UpperBoundMarker, Self::UpperBoundMarker) => Ordering::Equal,
            (Self::UpperBoundMarker, _) => Ordering::Greater,
            (_, Self::UpperBoundMarker) => Ordering::Less,

            (Self::Decimal(a), Self::Decimal(b)) => a.cmp(b),
            (Self::EntityKey(a), Self::EntityKey(b)) => a.cmp(b),
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::Nat(a), Self::Nat(b)) => a.cmp(b),
            (Self::Principal(a), Self::Principal(b)) => a.cmp(b),
            (Self::Text(a), Self::Text(b)) => a.cmp(b),
            (Self::Ulid(a), Self::Ulid(b)) => a.cmp(b),

            // Define an arbitrary but stable variant order fallback
            (a, b) => a.variant_rank().cmp(&b.variant_rank()),
        }
    }
}

impl PartialOrd for IndexValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
