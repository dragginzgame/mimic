mod error;

pub use error::ErrorTree;

use crate::orm::traits::EntityDyn;
use candid::CandidType;
use derive_more::IntoIterator;
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
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    Str,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
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

///
/// PrimitiveType
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
#[remain::sorted]
pub enum PrimitiveType {
    Blob,
    Bool,
    Decimal,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    Principal,
    String,
    U8,
    U16,
    U32,
    U64,
    U128,
    Ulid,
    Unit,
    Usize,
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
    String,
    Ulid,
}

///
/// SortDirection
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SortDirection {
    Asc,
    Desc,
}
