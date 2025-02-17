pub mod dynamic;
pub mod generic;

pub use dynamic::{DeleteBuilderDyn, DeleteExecutorDyn, DeleteQueryDyn};
pub use generic::{DeleteBuilder, DeleteExecutor, DeleteQuery};

use crate::{
    ThisError,
    db::{DbError, types::DataKey},
    query::resolver::ResolverError,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

///
/// DeleteError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum DeleteError {
    #[error("key not found: {0}")]
    KeyNotFound(DataKey),

    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    ResolverError(#[from] ResolverError),
}

///
/// DeleteRequest
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRequest {
    pub entity: String,
    pub key: Vec<String>,
}

///
/// DeleteResponse
///
/// keys : all the keys that have successfully been deleted
///

#[derive(CandidType, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct DeleteResponse(Vec<DataKey>);

impl DeleteResponse {
    #[must_use]
    pub fn new(keys: Vec<DataKey>) -> Self {
        Self(keys)
    }
}
