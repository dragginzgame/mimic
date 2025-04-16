use crate::{
    impl_storable_bounded,
    orm::{
        base::types::{Ulid, prim::ulid::UlidError},
        traits::{
            Filterable, Inner, Orderable, SortKeyValue, ValidateAuto, ValidateCustom, Visitable,
        },
    },
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{self},
    str::FromStr,
};

///
/// Relation
///

#[derive(
    CandidType, Clone, Debug, Default, Deref, DerefMut, Eq, PartialEq, Hash, Serialize, Deserialize,
)]
pub struct Relation(Vec<Ulid>);

impl Relation {
    #[must_use]
    pub fn contains(&self, ulid: &Ulid) -> bool {
        self.0.contains(ulid)
    }

    #[must_use]
    pub fn starts_with(&self, prefix: &[Ulid]) -> bool {
        self.0.starts_with(prefix)
    }

    pub fn push(&mut self, ulid: Ulid) {
        self.0.push(ulid);
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self
            .0
            .iter()
            .map(Ulid::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{formatted}]")
    }
}

impl Filterable for Relation {}

impl From<Ulid> for Relation {
    fn from(ulid: Ulid) -> Self {
        Self(vec![ulid])
    }
}

impl<U: Into<Ulid>> From<Vec<U>> for Relation {
    fn from(ulids: Vec<U>) -> Self {
        Self(ulids.into_iter().map(Into::into).collect())
    }
}

impl FromStr for Relation {
    type Err = UlidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ulid = Ulid::from_str(s)?;

        Ok(Relation(vec![ulid]))
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
    CandidType, Clone, Debug, Default, Deref, DerefMut, Eq, PartialEq, Serialize, Deserialize,
)]
pub struct RelationSet(HashSet<Relation>);

impl RelationSet {
    pub fn add(&mut self, relation: Relation) {
        self.0.insert(relation);
    }

    #[must_use]
    pub fn contains_ulid(&self, ulid: &Ulid) -> bool {
        self.0.iter().any(|r| r.contains(ulid))
    }

    #[must_use]
    pub fn contains_relation(&self, relation: &Relation) -> bool {
        self.0.contains(relation)
    }

    #[must_use]
    pub fn find_by_prefix(&self, prefix: &[Ulid]) -> Vec<&Relation> {
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

        write!(f, "[{}]", formatted)
    }
}
