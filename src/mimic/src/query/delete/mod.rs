pub mod generic;
pub mod path;

pub use generic::{DeleteBuilder, DeleteExecutor, DeleteQuery};
pub use path::{DeleteBuilderPath, DeleteExecutorPath, DeleteQueryPath};

use crate::{query::resolver::ResolverError, store::types::DataKey, Error, ThisError};
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
    ResolverError(ResolverError),
}

///
/// DeleteResponse
///
/// keys : all the keys that have successfully been deleted
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    keys: Vec<DataKey>,
}

impl DeleteResponse {
    // new
    const fn new(keys: Vec<DataKey>) -> Self {
        Self { keys }
    }

    // keys
    pub fn keys(self) -> Result<Vec<DataKey>, Error> {
        Ok(self.keys)
    }
}
