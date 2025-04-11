use crate::{
    impl_storable_bounded,
    orm::{
        base::types::Ulid,
        traits::{Filterable, Inner, Orderable, ValidateAuto, ValidateCustom, Visitable},
    },
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{self},
};

///
/// Relation
///

#[derive(
    CandidType, Clone, Debug, Default, Deref, DerefMut, Eq, PartialEq, Serialize, Deserialize,
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

impl Orderable for Relation {}

impl ValidateCustom for Relation {}

impl ValidateAuto for Relation {}

impl Visitable for Relation {}
