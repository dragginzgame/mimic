use crate::core::Key;
use candid::CandidType;
use serde::Serialize;

///
/// DbStats
///

#[derive(Debug, Default, CandidType, Serialize)]
pub struct DbStats {
    pub data_stores: Vec<StoreStats>,
    pub index_stores: Vec<IndexStats>,
}

///
/// StoreStats
///

#[derive(Debug, Default, CandidType, Serialize)]
pub struct StoreStats {
    pub path: String,
    pub entries: u64,
    pub min_key: Option<Key>,
    pub max_key: Option<Key>,
    pub memory_bytes: u64,
}

///
/// IndexStats
///

#[derive(Debug, Default, CandidType, Serialize)]
pub struct IndexStats {
    pub path: String,
    pub entries: u64,
    pub memory_bytes: u64,
}
