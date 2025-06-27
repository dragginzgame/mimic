use crate::{
    db::executor::ResolvedSelector,
    ops::{Value, traits::EntityKind},
    types::EntityKey,
};
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

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub enum Selector {
    #[default]
    All,
    Only,
    One(EntityKey),
    Many(Vec<EntityKey>),
    Prefix(EntityKey),
    Range(EntityKey, EntityKey),
}

impl Selector {
    #[must_use]
    pub fn resolve<E: EntityKind>(&self) -> ResolvedSelector {
        match self {
            Self::All => {
                let start = E::build_data_key(&[]);
                let end = start.create_upper_bound();

                ResolvedSelector::Range(start, end)
            }
            Self::Only => ResolvedSelector::One(E::build_data_key(&[])),
            Self::One(key) => ResolvedSelector::One(E::build_data_key(key)),
            Self::Many(keys) => {
                let keys = keys.iter().map(|k| E::build_data_key(k)).collect();

                ResolvedSelector::Many(keys)
            }
            Self::Prefix(prefix) => {
                let start = E::build_data_key(prefix);
                let end = start.create_upper_bound();

                ResolvedSelector::Range(start, end)
            }
            Self::Range(start, end) => {
                let start = E::build_data_key(start);
                let end = E::build_data_key(end);

                ResolvedSelector::Range(start, end)
            }
        }
    }
}

///
/// SortDirection
///

#[derive(CandidType, Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

///
/// Where
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct Where {
    pub matches: Vec<(String, Value)>,
}
