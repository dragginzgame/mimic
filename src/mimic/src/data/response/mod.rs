mod collection;

pub use collection::*;

use crate::data::store::{DataRow, SortKey};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// ResponseError
///

#[derive(Debug, ThisError)]
pub enum ResponseError {
    #[error(transparent)]
    CollectionError(#[from] collection::CollectionError),
}

///
/// LoadResponse
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub enum LoadResponse {
    Rows(Vec<DataRow>),
    Keys(Vec<SortKey>),
    Count(usize),
}

///
/// SaveResponse
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct SaveResponse {
    pub key: SortKey,
    pub created: u64,
    pub modified: u64,
}

///
/// DeleteResponse
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct DeleteResponse(pub Vec<SortKey>);
