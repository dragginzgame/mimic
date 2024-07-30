use crate::traits::Orderable;
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet as StdHashSet, hash::Hash};

///
/// HashSet
///
/// a wrapper around std::HashSet that the ORM uses
///

#[derive(
    CandidType, Clone, Debug, Default, Deref, DerefMut, Eq, PartialEq, Serialize, Deserialize,
)]
pub struct HashSet<V>(StdHashSet<V>)
where
    V: Hash + Eq;

impl<V> HashSet<V>
where
    V: Eq + Hash,
{
    #[must_use]
    pub fn new() -> Self {
        Self(StdHashSet::new())
    }
}

impl<V> IntoIterator for HashSet<V>
where
    V: Eq + Hash,
{
    type Item = V;
    type IntoIter = std::collections::hash_set::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<V> FromIterator<V> for HashSet<V>
where
    V: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a, V> IntoIterator for &'a HashSet<V>
where
    V: Eq + Hash,
{
    type Item = &'a V;
    type IntoIter = std::collections::hash_set::Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<V> Orderable for HashSet<V> where V: Eq + Hash {}
