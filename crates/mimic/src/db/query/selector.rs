use crate::{
    core::db::EntityKey,
    db::query::{BoundKind, QueryBound},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Selector
///
/// All    : no data key prefix, only works with top-level DataKeys
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
    /// Resolves the selector into a lower-level `ResolvedSelector`.
    ///
    /// This strips away intent-like variants (`All`, `Prefix`, etc.)
    /// and produces explicit keys and ranges, ready to be mapped
    /// to physical `DataKey`s and executed.
    #[must_use]
    pub fn resolve(&self) -> ResolvedSelector {
        match self {
            Self::All => {
                let key = EntityKey::from_values(&[]);

                let start = QueryBound {
                    key: key.clone(),
                    kind: BoundKind::Inclusive,
                };
                let end = QueryBound {
                    key: key.with_last_max(),
                    kind: BoundKind::Inclusive,
                };

                ResolvedSelector::Range(start, end)
            }

            Self::Only => ResolvedSelector::One(EntityKey::from_values(&[])),

            Self::One(key) => ResolvedSelector::One(key.clone()),

            Self::Many(keys) => ResolvedSelector::Many(keys.clone()),

            Self::Prefix(prefix) => {
                let start = QueryBound {
                    key: prefix.clone(),
                    kind: BoundKind::Inclusive,
                };
                let end = QueryBound {
                    key: prefix.with_last_max(),
                    kind: BoundKind::Inclusive,
                };
                ResolvedSelector::Range(start, end)
            }

            Self::Range(start_key, end_key) => {
                let start = QueryBound {
                    key: start_key.clone(),
                    kind: BoundKind::Inclusive,
                };
                let end = QueryBound {
                    key: end_key.clone(),
                    kind: BoundKind::Inclusive,
                };
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
    One(EntityKey),
    Many(Vec<EntityKey>),
    Range(QueryBound, QueryBound),
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
