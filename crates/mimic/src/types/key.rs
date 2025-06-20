use crate::{
    db::types::SortKey,
    ops::traits::{FieldOrderable, FieldQueryable, ValidateAuto, ValidateCustom, Visitable},
    types::Ulid,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{self, Display},
    str::FromStr,
};

///
/// Key
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
pub struct Key(pub Vec<String>);

impl Key {
    #[must_use]
    pub fn contains(&self, s: &str) -> bool {
        self.0.contains(&s.to_string())
    }

    #[must_use]
    pub fn starts_with(&self, prefix: &[&str]) -> bool {
        self.0.iter().zip(prefix).all(|(a, b)| a == b)
    }

    pub fn push(&mut self, s: &str) {
        self.0.push(s.to_string());
    }

    #[must_use]
    pub const fn as_vec(&self) -> &Vec<String> {
        &self.0
    }

    #[must_use]
    pub fn into_vec(self) -> Vec<String> {
        self.0
    }
}

impl AsRef<[String]> for Key {
    fn as_ref(&self) -> &[String] {
        &self.0
    }
}

impl Borrow<[String]> for Key {
    fn borrow(&self) -> &[String] {
        &self.0
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self.0.join(", ");
        write!(f, "[{formatted}]")
    }
}

impl FieldOrderable for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldQueryable for Key {
    fn to_query_value(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl From<Ulid> for Key {
    fn from(ulid: Ulid) -> Self {
        Self(vec![ulid.to_string()])
    }
}

impl From<SortKey> for Key {
    fn from(key: SortKey) -> Self {
        Self(
            key.parts()
                .into_iter()
                .map(|part| part.value.unwrap_or_default())
                .collect(),
        )
    }
}

impl From<&[&str]> for Key {
    fn from(ss: &[&str]) -> Self {
        Self(ss.iter().copied().map(ToString::to_string).collect())
    }
}

impl<S: ToString> From<Vec<S>> for Key {
    fn from(ss: Vec<S>) -> Self {
        Self(ss.into_iter().map(|s| s.to_string()).collect())
    }
}

impl FromStr for Key {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(vec![s.to_string()]))
    }
}

impl ValidateCustom for Key {}

impl ValidateAuto for Key {}

impl Visitable for Key {}

///
/// KeySet
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
pub struct KeySet(pub Vec<Key>);

impl KeySet {
    pub fn add(&mut self, key: Key) {
        if !self.0.contains(&key) {
            self.0.push(key);
        }
    }
}

impl fmt::Display for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self
            .0
            .iter()
            .map(Key::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{formatted}]")
    }
}

impl<K: Into<Key>> From<Vec<K>> for KeySet {
    fn from(vec: Vec<K>) -> Self {
        let keys = vec.into_iter().map(Into::into).collect();

        Self(keys)
    }
}
