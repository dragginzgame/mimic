use crate::{
    base::types::Ulid,
    traits::{Filterable, Orderable, SortKeyValue, ValidateAuto, ValidateCustom, Visitable},
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt, str::FromStr};

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
    Eq,
    PartialEq,
    Hash,
    IntoIterator,
    Serialize,
    Deserialize,
)]
pub struct Relation(Vec<String>);

impl Relation {
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
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self.0.join(", ");
        write!(f, "[{formatted}]")
    }
}

impl Filterable for Relation {}

impl From<Ulid> for Relation {
    fn from(ulid: Ulid) -> Self {
        Self(vec![ulid.to_string()])
    }
}

impl<S: ToString> From<Vec<S>> for Relation {
    fn from(vec: Vec<S>) -> Self {
        Self(vec.into_iter().map(|s| s.to_string()).collect())
    }
}

impl FromStr for Relation {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(vec![s.to_string()]))
    }
}

impl Orderable for Relation {}

impl SortKeyValue for Relation {}

impl ValidateCustom for Relation {}

impl ValidateAuto for Relation {}

impl Visitable for Relation {}

///
/// RelationSet
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
pub struct RelationSet(HashSet<Relation>);

impl RelationSet {
    pub fn add(&mut self, relation: Relation) {
        self.0.insert(relation);
    }

    #[must_use]
    pub fn contains_str(&self, s: &str) -> bool {
        self.0.iter().any(|r| r.contains(s))
    }

    #[must_use]
    pub fn contains_relation(&self, relation: &Relation) -> bool {
        self.0.contains(relation)
    }

    #[must_use]
    pub fn find_by_prefix(&self, prefix: &[&str]) -> Vec<&Relation> {
        self.0.iter().filter(|r| r.starts_with(prefix)).collect()
    }
}

impl fmt::Display for RelationSet {
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

impl<S: ToString> From<Vec<S>> for RelationSet {
    fn from(vec: Vec<S>) -> Self {
        let rels = vec
            .into_iter()
            .map(|s| Relation(vec![s.to_string()]))
            .collect();

        Self(rels)
    }
}
