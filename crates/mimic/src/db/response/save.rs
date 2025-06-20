use crate::db::types::SortKey;
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// SaveCollection
///

#[derive(Debug)]
pub struct SaveCollection(pub Vec<SaveRow>);

///
/// SaveResponse
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct SaveResponse(pub Vec<SaveRow>);

///
/// SaveRow
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct SaveRow {
    pub key: SortKey,
    pub created: u64,
    pub modified: u64,
}
