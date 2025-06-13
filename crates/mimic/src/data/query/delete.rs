use crate::{data::Selector, types::Key};
use candid::CandidType;
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
    pub fn one<K: Into<Key>>(self, key: K) -> DeleteQuery {
        let selector = Selector::One(key.into());

        DeleteQuery::new(selector)
    }

    // many
    #[must_use]
    pub fn many<K, I>(self, keys: I) -> DeleteQuery
    where
        K: Into<Key>,
        I: IntoIterator<Item = K>,
    {
        let keys = keys.into_iter().map(Into::into).collect();

        DeleteQuery::new(Selector::Many(keys))
    }
}
