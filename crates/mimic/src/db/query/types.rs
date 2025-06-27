use crate::{
    db::{executor::ResolvedSelector, store::DataKey},
    ops::{
        Value,
        traits::{EntityKind, FieldIndexValue, FieldOrderable},
        types::IndexValue,
    },
    types::Relation,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{self, Display},
};

///
/// EntityKey
///

#[derive(
    CandidType,
    Clone,
    Debug,
    Default,
    Deref,
    DerefMut,
    Deserialize,
    Eq,
    Hash,
    IntoIterator,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct EntityKey(pub Vec<IndexValue>);

impl EntityKey {
    #[must_use]
    pub const fn as_vec(&self) -> &Vec<IndexValue> {
        &self.0
    }

    #[must_use]
    pub fn into_vec(self) -> Vec<IndexValue> {
        self.0
    }
}

impl AsRef<[IndexValue]> for EntityKey {
    fn as_ref(&self) -> &[IndexValue] {
        &self.0
    }
}

impl Borrow<[IndexValue]> for EntityKey {
    fn borrow(&self) -> &[IndexValue] {
        &self.0
    }
}

impl Display for EntityKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted: Vec<String> = self.0.iter().map(ToString::to_string).collect();
        write!(f, "[{}]", formatted.join(", "))
    }
}

impl FieldOrderable for EntityKey {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl From<DataKey> for EntityKey {
    fn from(key: DataKey) -> Self {
        Self(
            key.parts()
                .into_iter()
                .filter_map(|part| part.value)
                .collect(),
        )
    }
}

impl From<IndexValue> for EntityKey {
    fn from(value: IndexValue) -> Self {
        Self(vec![value])
    }
}

impl From<Relation> for EntityKey {
    fn from(rel: Relation) -> Self {
        Self(rel.0)
    }
}

impl<T> From<Vec<T>> for EntityKey
where
    T: FieldIndexValue,
{
    fn from(values: Vec<T>) -> Self {
        Self(
            values
                .into_iter()
                .filter_map(|v| v.to_index_value())
                .collect(),
        )
    }
}

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
