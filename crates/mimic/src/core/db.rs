use crate::{
    core::{
        traits::{
            FieldSearchable, FieldSortable, FieldValue, ValidateAuto, ValidateCustom, Visitable,
        },
        types::Ulid,
        value::IndexValue,
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

    #[must_use]
    pub fn from_values(values: &[IndexValue]) -> Self {
        Self(values.to_vec())
    }

    /// Returns a new key where the last component is replaced with its sentinel max.
    /// If the key is empty, returns a single-element key containing `IndexValue::sentinel_max()`.
    #[must_use]
    pub fn with_last_max(&self) -> Self {
        if self.0.is_empty() {
            // Treat empty key as the lowest bound — return a high upper bound
            return Self(vec![IndexValue::MAX]);
        }

        let mut values = self.0.clone();
        if let Some(last) = values.last_mut() {
            *last = last.sentinel_max();
        }

        Self(values)
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

impl FieldValue for EntityKey {}

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
