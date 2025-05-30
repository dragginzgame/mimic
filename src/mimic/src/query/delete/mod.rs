mod dynamic;

pub use dynamic::*;

use crate::db::types::SortKey;
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// DeleteError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum DeleteError {
    #[error("undefined delete query")]
    Undefined,
}

///
/// DeleteMethod
///
/// One  : one key
/// Many : many keys
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub enum DeleteMethod {
    #[default]
    Undefined,
    One(Vec<String>),
    Many(Vec<Vec<String>>),
}

///
/// DeleteQuery
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct DeleteQuery {
    path: String,
    method: DeleteMethod,
}

impl DeleteQuery {
    // new
    #[must_use]
    pub fn new(path: &str, method: DeleteMethod) -> Self {
        Self {
            path: path.to_string(),
            method,
        }
    }
}

///
/// DeleteResponse
///

#[derive(CandidType, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct DeleteResponse(Vec<SortKey>);
