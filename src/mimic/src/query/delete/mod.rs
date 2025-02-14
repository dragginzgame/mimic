pub mod generic;
pub mod path;

pub use generic::{DeleteBuilder, DeleteExecutor, DeleteQuery};
pub use path::{DeleteBuilderPath, DeleteExecutorPath, DeleteQueryPath};

use crate::{
    db::{types::DataKey, DbError},
    query::resolver::ResolverError,
    ThisError,
};
use candid::CandidType;
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

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub enum DeleteResponse {
    Keys(Vec<DataKey>),
}
