use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Selector
///
/// All    : no sort key prefix, only works with top-level Sort Keys
/// Only   : for entities that have no keys
/// One    : returns one row by composite key
/// Many   : returns many rows (from many composite keys)
/// Prefix : like all but we're asking for the composite key prefix
///          so Pet (Character=1) will return the Pets from Character 1
/// Range  : user-defined range, ie. Item=1000 Item=1500
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub enum Selector {
    #[default]
    All,
    Only,
    One(Vec<String>),
    Many(Vec<Vec<String>>),
    Prefix(Vec<String>),
    Range(Vec<String>, Vec<String>),
}

///
/// Where
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Where {
    pub matches: Vec<(String, String)>,
}

impl<S: ToString> From<Vec<(S, S)>> for Where {
    fn from(pairs: Vec<(S, S)>) -> Self {
        Self {
            matches: pairs
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}
