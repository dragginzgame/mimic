use crate::{
    data::store::SortKey,
    traits::{Orderable, SortKeyPart, ValidateAuto, ValidateCustom, Visitable},
    types::Ulid,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashSet, fmt, str::FromStr};

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
    Eq,
    PartialEq,
    Hash,
    IntoIterator,
    Serialize,
    Deserialize,
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

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self.0.join(", ");
        write!(f, "[{formatted}]")
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
            key.0
                .into_iter()
                .map(|(_, opt)| opt.unwrap_or_default())
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

impl Orderable for Key {}

impl SortKeyPart for Key {}

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
pub struct KeySet(HashSet<Key>);

impl KeySet {
    pub fn add(&mut self, key: Key) {
        self.0.insert(key);
    }

    #[must_use]
    pub fn contains_key(&self, key: &Key) -> bool {
        self.0.contains(key)
    }

    #[must_use]
    pub fn find_by_prefix(&self, prefix: &[&str]) -> Vec<&Key> {
        self.0.iter().filter(|r| r.starts_with(prefix)).collect()
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

impl<S: ToString> From<Vec<S>> for KeySet {
    fn from(vec: Vec<S>) -> Self {
        let keys = vec.into_iter().map(|s| Key(vec![s.to_string()])).collect();

        Self(keys)
    }
}

impl<'a> IntoIterator for &'a KeySet {
    type Item = &'a Key;
    type IntoIter = std::collections::hash_set::Iter<'a, Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
