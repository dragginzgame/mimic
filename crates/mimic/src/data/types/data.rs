use crate::data::types::SortKey;
use candid::CandidType;
use icu::impl_storable_unbounded;
use serde::{Deserialize, Serialize};

///
/// DataRow
/// the data B-tree key and value pair
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DataRow {
    pub key: SortKey,
    pub value: DataValue,
}

impl DataRow {
    #[must_use]
    pub const fn new(key: SortKey, value: DataValue) -> Self {
        Self { key, value }
    }
}

///
/// DataValue
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DataValue {
    pub bytes: Vec<u8>,
    pub path: String,
    pub metadata: Metadata,
}

impl_storable_unbounded!(DataValue);

///
/// Metadata
///

#[derive(CandidType, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Metadata {
    pub created: u64,
    pub modified: u64,
}
