use crate::{
    core::{
        traits::{
            FieldSearchable, FieldSortable, FieldValue, ValidateAuto, ValidateCustom, Visitable,
        },
        types::Ulid,
        value::{IndexValue, Value},
    },
    db::store::DataKey,
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

impl FieldSearchable for EntityKey {}

impl FieldSortable for EntityKey {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldValue for EntityKey {
    fn to_value(&self) -> Value {
        Value::EntityKey(self.clone())
    }
}

impl From<DataKey> for EntityKey {
    fn from(key: DataKey) -> Self {
        let values = key
            .parts()
            .into_iter()
            .filter_map(|part| part.value)
            .collect();

        Self(values)
    }
}

impl From<IndexValue> for EntityKey {
    fn from(value: IndexValue) -> Self {
        Self(vec![value])
    }
}

impl From<Ulid> for EntityKey {
    fn from(ulid: Ulid) -> Self {
        Self(vec![ulid.into()])
    }
}

impl<T> From<Vec<T>> for EntityKey
where
    T: Into<IndexValue>,
{
    fn from(values: Vec<T>) -> Self {
        Self(values.into_iter().map(Into::into).collect())
    }
}

impl ValidateAuto for EntityKey {}

impl ValidateCustom for EntityKey {}

impl Visitable for EntityKey {}

///
/// EntityKeys
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
pub struct EntityKeys(pub Vec<EntityKey>);

impl EntityKeys {
    pub fn add(&mut self, key: EntityKey) {
        if !self.0.contains(&key) {
            self.0.push(key);
        }
    }
}

impl Display for EntityKeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self
            .0
            .iter()
            .map(EntityKey::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{formatted}]")
    }
}

impl<K: Into<EntityKey>> From<Vec<K>> for EntityKeys {
    fn from(vec: Vec<K>) -> Self {
        let keys = vec.into_iter().map(Into::into).collect();

        Self(keys)
    }
}
