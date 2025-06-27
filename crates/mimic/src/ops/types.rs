use crate::types::{Decimal, Principal, Ulid};
use candid::CandidType;
use derive_more::{Deref, DerefMut, Display};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap};

///
/// Values
///

#[derive(Debug, Deref, DerefMut)]
pub struct Values(HashMap<String, Option<Value>>);

impl Values {
    /// Returns Some(values) if all fields are present, or None if any are missing or None
    #[must_use]
    pub fn collect_all(&self, fields: &[String]) -> Option<Vec<Value>> {
        let mut values = Vec::with_capacity(fields.len());

        for field in fields {
            match self.get(field) {
                Some(Some(v)) => values.push(v.clone()),
                _ => return None, // required field missing or None
            }
        }

        Some(values)
    }

    /// Checks if all given fields are present
    #[must_use]
    pub fn has_all(&self, fields: &[String]) -> bool {
        fields.iter().all(|f| self.contains_key(f))
    }
}

///
/// Value
/// can be searched or used in WHERE statements
///

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Value {
    Bool(bool),
    Blob(Vec<u8>),
    Decimal(Decimal),
    Int128(i128),
    Nat128(u128),
    Text(String),
    Ulid(Ulid),
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<Decimal> for Value {
    fn from(v: Decimal) -> Self {
        Self::Decimal(v)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

///
/// IndexValues
///

#[derive(Debug, Deref, DerefMut)]
pub struct IndexValues(HashMap<String, Option<IndexValue>>);

impl IndexValues {
    /// Returns Some(values) if all fields are present, or None if any are missing or None
    #[must_use]
    pub fn collect_all(&self, fields: &[String]) -> Option<Vec<IndexValue>> {
        let mut values = Vec::with_capacity(fields.len());

        for field in fields {
            match self.get(field) {
                Some(Some(v)) => values.push(v.clone()),
                _ => return None, // required field missing or None
            }
        }

        Some(values)
    }

    /// Checks if all given fields are present
    #[must_use]
    pub fn has_all(&self, fields: &[String]) -> bool {
        fields.iter().all(|f| self.contains_key(f))
    }
}

///
/// IndexValue
///

#[derive(CandidType, Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
pub enum IndexValue {
    Decimal(Decimal),
    Int128(i128),
    Nat128(u128),
    Principal(Principal),
    Text(String),
    Ulid(Ulid),
    UpperBoundMarker,
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

            (Int128(a), Int128(b)) => a.cmp(b),
            (Nat128(a), Nat128(b)) => a.cmp(b),
            (Text(a), Text(b)) => a.cmp(b),

            // Define an arbitrary but stable variant order fallback
            (a, b) => variant_rank(a).cmp(&variant_rank(b)),
        }
    }
}

fn variant_rank(v: &IndexValue) -> u8 {
    use IndexValue::*;

    match v {
        Decimal(_) => 1,
        Int128(_) => 2,
        Nat128(_) => 3,
        Principal(_) => 4,
        Text(_) => 5,
        Ulid(_) => 6,
        UpperBoundMarker => 7,
    }
}

impl From<Decimal> for IndexValue {
    fn from(v: Decimal) -> Self {
        Self::Decimal(v)
    }
}

impl From<String> for IndexValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for IndexValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}
