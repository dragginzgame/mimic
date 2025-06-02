mod dynamic;
mod generic;

pub use dynamic::*;
pub use generic::*;

use crate::{
    db::types::{DataRow, SortKey},
    types::prim::Relation,
};
use candid::CandidType;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap};

///
/// LoadFormat
///

#[derive(CandidType, Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum LoadFormat {
    #[default]
    Rows,
    Keys,
    Count,
}

///
/// LoadResponse
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub enum LoadResponse {
    Rows(Vec<DataRow>),
    Keys(Vec<SortKey>),
    Count(usize),
}

///
/// LoadMap
/// a HashMap indexed by id to provide an indexed alternative
/// to Vec<Row>
///

#[derive(Debug, Deref)]
pub struct LoadMap<T>(HashMap<Relation, T>);

impl<T> LoadMap<T> {
    // from_pairs
    pub fn from_pairs<I>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (Relation, T)>,
    {
        let map: HashMap<Relation, T> = pairs.into_iter().collect();

        Self(map)
    }

    // get
    pub fn get<R: Borrow<Relation>>(&self, r: R) -> Option<&T> {
        self.0.get(r.borrow())
    }

    // get_many
    pub fn get_many<Q, I>(&self, keys: I) -> Vec<&T>
    where
        Q: Borrow<Relation>,
        I: IntoIterator<Item = Q>,
    {
        keys.into_iter()
            .filter_map(|k| self.0.get(k.borrow()))
            .collect()
    }
}
