use crate::{
    db::query::EntityKey,
    ops::{
        IndexValue, Value,
        traits::{
            FieldIndexValue, FieldOrderable, FieldSearch, FieldValue, ValidateAuto, ValidateCustom,
            Visitable,
        },
    },
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
/// Relation
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
pub struct Relation(pub Vec<IndexValue>);

impl Relation {
    #[must_use]
    pub const fn as_vec(&self) -> &Vec<IndexValue> {
        &self.0
    }

    #[must_use]
    pub fn into_vec(self) -> Vec<IndexValue> {
        self.0
    }
}

impl AsRef<[IndexValue]> for Relation {
    fn as_ref(&self) -> &[IndexValue] {
        &self.0
    }
}

impl Borrow<[IndexValue]> for Relation {
    fn borrow(&self) -> &[IndexValue] {
        &self.0
    }
}

impl Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted: Vec<String> = self.0.iter().map(ToString::to_string).collect();
        write!(f, "[{}]", formatted.join(", "))
    }
}

impl FieldIndexValue for Relation {
    fn to_index_value(&self) -> Option<IndexValue> {
        Some(IndexValue::Relation(self.clone()))
    }
}

impl FieldOrderable for Relation {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldValue for Relation {
    fn to_value(&self) -> Option<Value> {
        Some(Value::Relation(self.clone()))
    }
}

impl FieldSearch for Relation {}

impl From<EntityKey> for Relation {
    fn from(key: EntityKey) -> Self {
        Self(key.0)
    }
}

impl ValidateCustom for Relation {}

impl ValidateAuto for Relation {}

impl Visitable for Relation {}

///
/// RelationMany
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
    IntoIterator,
    Serialize,
    Deserialize,
)]
pub struct RelationMany(pub Vec<Relation>);

impl RelationMany {
    pub fn add(&mut self, rel: Relation) {
        if !self.0.contains(&rel) {
            self.0.push(rel);
        }
    }
}

impl Display for RelationMany {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self
            .0
            .iter()
            .map(Relation::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{formatted}]")
    }
}

impl<K: Into<Relation>> From<Vec<K>> for RelationMany {
    fn from(vec: Vec<K>) -> Self {
        let keys = vec.into_iter().map(Into::into).collect();

        Self(keys)
    }
}
