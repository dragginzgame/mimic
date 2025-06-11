mod dynamic;
mod generic;

pub use dynamic::*;
pub use generic::*;

use crate::types::Key;
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
/// LoadMap
/// a HashMap indexed by id to provide an indexed alternative
/// to Vec<Row>
///

#[derive(Debug, Deref)]
pub struct LoadMap<T>(HashMap<Key, T>);

impl<T> LoadMap<T> {
    // from_pairs
    pub fn from_pairs<I>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (Key, T)>,
    {
        let map: HashMap<Key, T> = pairs.into_iter().collect();

        Self(map)
    }

    // get
    pub fn get<R: Borrow<Key>>(&self, r: R) -> Option<&T> {
        self.0.get(r.borrow())
    }

    // get_many
    pub fn get_many<Q, I>(&self, keys: I) -> Vec<&T>
    where
        Q: Borrow<Key>,
        I: IntoIterator<Item = Q>,
    {
        keys.into_iter()
            .filter_map(|k| self.0.get(k.borrow()))
            .collect()
    }
}
