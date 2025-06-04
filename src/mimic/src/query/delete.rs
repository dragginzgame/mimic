use crate::{db::types::SortKey, query::Selector};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

///
/// DeleteQuery
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteQuery {
    pub selector: Selector,
}

impl DeleteQuery {
    // new
    #[must_use]
    pub const fn new(selector: Selector) -> Self {
        Self { selector }
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
    pub fn new() -> Self {
        Self::default()
    }

    // one
    pub fn one<S: ToString>(self, ck: &[S]) -> DeleteQuery {
        let key = ck.iter().map(ToString::to_string).collect();
        let selector = Selector::One(key);

        DeleteQuery::new(selector)
    }

    // many
    #[must_use]
    pub fn many<I, S>(self, cks: I) -> DeleteQuery
    where
        I: IntoIterator,
        I::Item: IntoIterator<Item = S>,
        S: ToString,
    {
        let keys: Vec<Vec<String>> = cks
            .into_iter()
            .map(|inner| inner.into_iter().map(|s| s.to_string()).collect())
            .collect();

        DeleteQuery::new(Selector::Many(keys))
    }
}
