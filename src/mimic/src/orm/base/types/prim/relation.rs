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
