use candid::CandidType;
use derive_more::{Deref, DerefMut};
use icu::impl_storable_unbounded;
use serde::{Deserialize, Serialize};

///
/// CompositeKey
///

#[derive(
    CandidType,
    Clone,
    Debug,
    Default,
    Deref,
    DerefMut,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct CompositeKey(pub Vec<String>);

impl<S: ToString> From<Vec<S>> for CompositeKey {
    fn from(v: Vec<S>) -> Self {
        Self(v.into_iter().map(|x| x.to_string()).collect())
    }
}

impl_storable_unbounded!(CompositeKey);

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
    One(CompositeKey),
    Many(Vec<CompositeKey>),
    Prefix(CompositeKey),
    Range(CompositeKey, CompositeKey),
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
