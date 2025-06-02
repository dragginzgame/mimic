use crate::{db::types::SortKey, query::Selector};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

///
/// DeleteQuery
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteQuery {
    pub path: String,
    pub selector: Selector,
}

impl DeleteQuery {
    // new
    #[must_use]
    pub fn new(path: &str, selector: Selector) -> Self {
        Self {
            path: path.to_string(),
            selector,
        }
    }
}

///
/// DeleteResponse
///

#[derive(CandidType, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct DeleteResponse(pub Vec<SortKey>);

///
/// DeleteQueryBuilder
///

#[derive(Debug, Default)]
pub struct DeleteQueryBuilder {}

impl DeleteQueryBuilder {
    // new
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    // one
    pub fn one<S: ToString>(self, path: &str, ck: &[S]) -> DeleteQuery {
        let key = ck.iter().map(ToString::to_string).collect();
        let selector = Selector::One(key);

        DeleteQuery::new(path, selector)
    }

    // many
    #[must_use]
    pub fn many<S: ToString>(self, path: &str, ck: &[Vec<S>]) -> DeleteQuery {
        let keys: Vec<Vec<String>> = ck
            .iter()
            .map(|inner_vec| inner_vec.iter().map(ToString::to_string).collect())
            .collect();
        let selector = Selector::Many(keys);

        DeleteQuery::new(path, selector)
    }
}
