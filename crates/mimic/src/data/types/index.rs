use crate::types::Key;
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use icu::{impl_storable_bounded, impl_storable_unbounded};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{self, Display},
};

///
/// STORAGE & API TYPES
///

///
/// IndexKey
///

#[derive(
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct IndexKey {
    pub entity_id: u64,
    pub fields: Vec<String>,
    pub values: Vec<String>,
}

impl IndexKey {
    #[must_use]
    pub fn new(entity_id: u64, fields: &[&str], values: &[&str]) -> Self {
        Self {
            entity_id,
            fields: fields.iter().map(|s| s.to_string()).collect(),
            values: values.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Display for IndexKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({} [{}] [{}])",
            self.entity_id,
            self.fields.join(", "),
            self.values.join(", ")
        )
    }
}

impl_storable_bounded!(IndexKey, 256, false);

///
/// IndexValue
///

#[derive(CandidType, Clone, Debug, Default, Deref, DerefMut, Serialize, Deserialize)]
pub struct IndexValue(pub HashSet<Key>);

impl IndexValue {
    #[must_use]
    pub fn from_key(key: Key) -> Self {
        Self::from(vec![key])
    }
}

impl<S: Into<Key>> From<Vec<S>> for IndexValue {
    fn from(v: Vec<S>) -> Self {
        Self(v.into_iter().map(Into::into).collect())
    }
}

impl_storable_unbounded!(IndexValue);
