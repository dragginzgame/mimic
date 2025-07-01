use crate::core::types::{Decimal, E8s, E18s, Principal, Ulid};
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
            Self::Int(i) => Some(IndexValue::Int(i)),
            Self::Nat(n) => Some(IndexValue::Nat(n)),
            Self::Principal(p) => Some(IndexValue::Principal(p)),
            Self::Ulid(u) => Some(IndexValue::Ulid(u)),
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
    UpperBoundMarker,
}

impl IndexValue {
    const fn variant_rank(&self) -> u8 {
        match self {
            Self::Int(_) => 0,
            Self::Nat(_) => 1,
            Self::Principal(_) => 2,
            Self::Ulid(_) => 3,
            Self::UpperBoundMarker => u8::MAX,
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

            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::Nat(a), Self::Nat(b)) => a.cmp(b),
            (Self::Principal(a), Self::Principal(b)) => a.cmp(b),
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

///
/// IndexValueSingle
/// helper to translate from/to PrimaryKeys
///

pub trait IndexValueSingle {
    fn to_value(&self) -> IndexValue;
    fn from_value(value: IndexValue) -> Self;
}

impl IndexValueSingle for [IndexValue; 1] {
    fn to_value(&self) -> IndexValue {
        self[0]
    }

    fn from_value(value: IndexValue) -> Self {
        [value]
    }
}
