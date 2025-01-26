pub mod dynamic;
pub mod r#static;

pub use dynamic::{DeleteBuilder, DeleteExecutor, DeleteQuery};
pub use r#static::{EDeleteBuilder, EDeleteExecutor, EDeleteQuery};

use crate::db::{db::DbError, query::resolver::ResolverError, types::DataKey};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// DeleteError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum DeleteError {
    #[snafu(transparent)]
    Db { source: DbError },

    #[snafu(transparent)]
    Resolver { source: ResolverError },
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
    pub fn keys(self) -> Result<Vec<DataKey>, DeleteError> {
        Ok(self.keys)
    }
}
