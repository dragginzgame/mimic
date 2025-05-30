mod error;

pub use error::ErrorTree;

use crate::traits::EntityDyn;
use derive_more::IntoIterator;

use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Cardinality
///

#[derive(CandidType, Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Cardinality {
    One,
    Opt,
    Many,
}

///
/// ConstantType
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
#[remain::sorted]
pub enum ConstantType {
    Bool,
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Nat128,
    Str,
}

///
/// PrimitiveType
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
#[remain::sorted]
pub enum PrimitiveType {
    Blob,
    Bool,
    Decimal,
    Float32,
    Float64,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Nat,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Nat128,
    Principal,
    Relation,
    RelationSet,
    Text,
    Ulid,
    Unit,
}

///
/// PrimitiveGroup
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
#[remain::sorted]
pub enum PrimitiveGroup {
    Blob,
    Bool,
    Decimal,
    Float,
    Integer,
    Relation,
    Text,
    Ulid,
    Unit,
}

///
/// SortDirection
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SortDirection {
    Asc,
    Desc,
}

///
/// FixtureList
///

#[derive(Debug, Default, IntoIterator)]
pub struct FixtureList(pub Vec<Box<dyn EntityDyn + 'static>>);

impl FixtureList {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<T: EntityDyn + 'static>(&mut self, entity: T) {
        self.0.push(Box::new(entity));
    }

    pub fn extend(&mut self, list: Self) {
        for entity in list {
            self.0.push(entity);
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<Box<dyn EntityDyn>>> for FixtureList {
    fn into(self) -> Vec<Box<dyn EntityDyn>> {
        self.0
    }
}
