use crate::{data::types::SortKey, traits::EntityKind, types::Key};
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

impl Selector {
    #[must_use]
    pub fn resolve<E: EntityKind>(&self) -> ResolvedSelector {
        match self {
            Selector::All => {
                let start = E::build_sort_key(&[]);
                let end = start.create_upper_bound();

                ResolvedSelector::Range(start, end)
            }
            Selector::Only => ResolvedSelector::One(E::build_sort_key(&[])),
            Selector::One(key) => ResolvedSelector::One(E::build_sort_key(key)),
            Selector::Many(keys) => {
                let keys = keys.iter().map(|k| E::build_sort_key(k)).collect();

                ResolvedSelector::Many(keys)
            }
            Selector::Prefix(prefix) => {
                let start = E::build_sort_key(prefix);
                let end = start.create_upper_bound();

                ResolvedSelector::Range(start, end)
            }
            Selector::Range(start, end) => {
                let start = E::build_sort_key(start);
                let end = E::build_sort_key(end);

                ResolvedSelector::Range(start, end)
            }
        }
    }
}

///
/// ResolvedSelector
///

#[derive(Debug)]
pub enum ResolvedSelector {
    One(SortKey),
    Many(Vec<SortKey>),
    Range(SortKey, SortKey),
}
