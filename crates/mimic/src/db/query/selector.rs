use crate::{
    core::{Key, traits::EntityKind},
    db::{
        query::{BoundKind, QueryBound, QueryRange, QueryShape},
        store::DataKey,
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Selector
///
/// All    : all rows
/// One    : returns one row by composite key
/// Many   : returns many rows (from many keys)
/// Range  : user-defined range, ie. Item=1000 Item=1500
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub enum Selector {
    #[default]
    All,
    One(Key),
    Many(Vec<Key>),
    Range(Key, Key),
}

impl Selector {
    /// Resolves the selector into a lower-level `QueryShape`.
    ///
    /// This strips away intent-like variants (`All`, `Prefix`, etc.)
    /// and produces explicit keys and ranges, ready to be mapped
    /// to physical `DataKey`s and executed.
    #[must_use]
    pub fn resolve<E: EntityKind>(&self) -> QueryShape {
        match self {
            Self::All => QueryShape::All,

            Self::One(key) => QueryShape::One(DataKey::with_entity::<E>(*key)),

            Self::Many(keys) => {
                let data_keys = keys.iter().map(|k| DataKey::with_entity::<E>(*k)).collect();

                QueryShape::Many(data_keys)
            }

            Self::Range(start_key, end_key) => {
                let start = QueryBound {
                    key: DataKey::with_entity::<E>(*start_key),
                    kind: BoundKind::Inclusive,
                };
                let end = QueryBound {
                    key: DataKey::with_entity::<E>(*end_key),
                    kind: BoundKind::Inclusive,
                };

                QueryShape::Range(QueryRange::new(start, end))
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
