use crate::core::types::EntityKey;
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

#[derive(CandidType, Debug, Deserialize, Serialize)]
pub struct DeleteResponse(pub Vec<DeleteRow>);

///
/// DeleteRow
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct DeleteRow {
    pub key: EntityKey,
}

impl DeleteRow {
    #[must_use]
    pub const fn new(key: EntityKey) -> Self {
        Self { key }
    }
}
