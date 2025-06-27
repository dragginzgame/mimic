use crate::types::{Decimal, Principal, Relation, Ulid};
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
    Float(f64),
    Int(i64),
    Nat(u64),
    Relation(Relation),
    Text(String),
    Ulid(Ulid),
}

///
/// IndexValue
///

#[derive(CandidType, Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub enum IndexValue {
    Decimal(Decimal),
    Int(i64),
    Nat(u64),
    Principal(Principal),
    Relation(Relation),
    Text(String),
    Ulid(Ulid),
    UpperBoundMarker,
}

impl IndexValue {
    fn variant_rank(&self) -> u8 {
        use IndexValue::*;

        match self {
            Decimal(_) => 0,
            Int(_) => 1,
            Nat(_) => 2,
            Principal(_) => 3,
            Relation(_) => 4,
            Text(_) => 5,
            Ulid(_) => 6,
            UpperBoundMarker => u8::MAX,
        }
    }
}

impl PartialOrd for IndexValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IndexValue {
    fn cmp(&self, other: &Self) -> Ordering {
        use IndexValue::*;

        match (self, other) {
            (UpperBoundMarker, UpperBoundMarker) => Ordering::Equal,
            (UpperBoundMarker, _) => Ordering::Greater,
            (_, UpperBoundMarker) => Ordering::Less,

            (Decimal(a), Decimal(b)) => a.cmp(b),
            (Int(a), Int(b)) => a.cmp(b),
            (Nat(a), Nat(b)) => a.cmp(b),
            (Principal(a), Principal(b)) => a.cmp(b),
            (Relation(a), Relation(b)) => a.cmp(b),
            (Text(a), Text(b)) => a.cmp(b),
            (Ulid(a), Ulid(b)) => a.cmp(b),

            // Define an arbitrary but stable variant order fallback
            (a, b) => a.variant_rank().cmp(&b.variant_rank()),
        }
    }
}
