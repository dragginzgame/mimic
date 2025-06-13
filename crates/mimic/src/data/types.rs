use crate::types::Key;
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Selector
///
/// All    : no sort key prefix, only works with top-level Sort Keys
/// Only   : for entities that have no keys
/// One    : returns one row by composite key
/// Many   : returns many rows (from many keys)
/// Prefix : like all but we're asking for the key prefix
///          so Pet (Character=1) will return the Pets from Character 1
/// Range  : user-defined range, ie. Item=1000 Item=1500
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub enum Selector {
    #[default]
    All,
    Only,
    One(Key),
    Many(Vec<Key>),
    Prefix(Key),
    Range(Key, Key),
}

///
/// SortDirection
///

#[derive(CandidType, Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

///
/// Where
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Where {
    pub matches: Vec<(String, String)>,
}

impl<K: ToString, V: ToString> From<Vec<(K, V)>> for Where {
    fn from(pairs: Vec<(K, V)>) -> Self {
        Self {
            matches: pairs
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}
