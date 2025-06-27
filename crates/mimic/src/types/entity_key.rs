use crate::{
    db::store::DataKey,
    ops::{
        IndexValue, Value,
        traits::{
            FieldIndexValue, FieldOrderable, FieldSearch, FieldValue, ValidateAuto, ValidateCustom,
            Visitable,
        },
    },
    types::Ulid,
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

impl FieldIndexValue for EntityKey {
    fn to_index_value(&self) -> Option<IndexValue> {
        Some(IndexValue::EntityKey(self.clone()))
    }
}

impl FieldOrderable for EntityKey {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldValue for EntityKey {
    fn to_value(&self) -> Option<Value> {
        Some(Value::EntityKey(self.clone()))
    }
}

impl FieldSearch for EntityKey {}

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

impl ValidateCustom for EntityKey {}

impl ValidateAuto for EntityKey {}

impl Visitable for EntityKey {}
