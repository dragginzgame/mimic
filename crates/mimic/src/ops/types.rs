use crate::types::{Decimal, EntityKey, Principal, Ulid};
use candid::CandidType;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

///
/// Value
/// can be searched or used in WHERE statements
///

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Value {
    Bool(bool),
    Decimal(Decimal),
    EntityKey(EntityKey),
    Float(f64),
    Int(i64),
    Nat(u64),
    Principal(Principal),
    Text(String),
    Ulid(Ulid),
}

macro_rules! impl_from_for_value {
    ( $( $type:ty => $variant:ident as $cast:ty ),* $(,)? ) => {
        $(
            impl From<$type> for Value {
                fn from(v: $type) -> Self {
                    Self::$variant(v as $cast)
                }
            }
        )*
    };
}

impl_from_for_value! {
    bool => Bool as bool,
    Decimal => Decimal as Decimal,
    EntityKey => EntityKey as EntityKey,
    f32 => Float as f64,
    f64 => Float as f64,
    i8 => Int as i64,
    i16 => Int as i64,
    i32 => Int as i64,
    i64 => Int as i64,
    Principal => Principal as Principal,
    String => Text as String,
    Ulid => Ulid as Ulid,
    u8 => Nat as u64,
    u16 => Nat as u64,
    u32 => Nat as u64,
    u64 => Nat as u64,
}

impl From<candid::Principal> for Value {
    fn from(p: candid::Principal) -> Self {
        Self::Principal(p.into())
    }
}

///
/// IndexValue
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

macro_rules! impl_from_for_index_value {
    ( $( $type:ty => $variant:ident as $cast:ty ),* $(,)? ) => {
        $(
            impl From<$type> for IndexValue {
                fn from(v: $type) -> Self {
                    Self::$variant(v as $cast)
                }
            }
        )*
    };
}

impl_from_for_index_value! {
    Decimal => Decimal as Decimal,
    EntityKey => EntityKey as EntityKey,
    i8 => Int as i64,
    i16 => Int as i64,
    i32 => Int as i64,
    i64 => Int as i64,
    Principal => Principal as Principal,
    String => Text as String,
    Ulid => Ulid as Ulid,
    u8 => Nat as u64,
    u16 => Nat as u64,
    u32 => Nat as u64,
    u64 => Nat as u64,
}

impl From<candid::Principal> for IndexValue {
    fn from(p: candid::Principal) -> Self {
        Self::Principal(p.into())
    }
}

impl PartialOrd for IndexValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
