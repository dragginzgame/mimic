use crate::data::types::SortKey;
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// DeleteCollection
///

#[derive(Debug)]
pub struct DeleteCollection(pub Vec<DeleteRow>);

///
/// DeleteResponse
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct DeleteResponse(pub Vec<DeleteRow>);

///
/// DeleteRow
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRow {
    pub key: SortKey,
}

impl DeleteRow {
    #[must_use]
    pub const fn new(key: SortKey) -> Self {
        Self { key }
    }
}
