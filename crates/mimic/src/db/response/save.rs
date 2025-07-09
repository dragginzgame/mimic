use crate::core::Key;
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
    pub key: Key,
    pub created: u64,
    pub modified: u64,
}
