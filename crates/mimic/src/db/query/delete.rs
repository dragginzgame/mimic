use crate::db::query::{EntityKey, Selector};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// DeleteQuery
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
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
    pub fn one<K: Into<EntityKey>>(self, key: K) -> DeleteQuery {
        let selector = Selector::One(key.into());

        DeleteQuery::new(selector)
    }

    // many
    #[must_use]
    pub fn many<K, I>(self, keys: I) -> DeleteQuery
    where
        K: Into<EntityKey>,
        I: IntoIterator<Item = K>,
    {
        let keys = keys.into_iter().map(Into::into).collect();

        DeleteQuery::new(Selector::Many(keys))
    }
}
