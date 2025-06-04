use crate::{db::types::SortKey, query::Selector, traits::EntityKind};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

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
pub struct DeleteQueryBuilder<E>
where
    E: EntityKind,
{
    phantom: PhantomData<E>,
}

impl<E> DeleteQueryBuilder<E>
where
    E: EntityKind,
{
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
    pub fn many<S: ToString>(self, ck: &[Vec<S>]) -> DeleteQuery {
        let keys: Vec<Vec<String>> = ck
            .iter()
            .map(|inner_vec| inner_vec.iter().map(ToString::to_string).collect())
            .collect();
        let selector = Selector::Many(keys);

        DeleteQuery::new(selector)
    }
}
