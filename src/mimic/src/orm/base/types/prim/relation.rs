use crate::orm::{
    base::types::prim::Ulid,
    traits::{
        Filterable, Inner, Orderable, PrimaryKey, SanitizeAuto, SanitizeManual, ValidateAuto,
        ValidateManual, Visitable,
    },
};
use candid::CandidType;
use derive_more::{Add, AddAssign, Deref, DerefMut, FromStr, Sub, SubAssign};
use num_traits::{FromPrimitive, NumCast, ToPrimitive};
use rust_decimal::Decimal as WrappedDecimal;
use serde::{ser::Error, Deserialize, Serialize};
use std::{cmp::Ordering, fmt};

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
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct Relation(Vec<Ulid>);

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ulids = self
            .0
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{ulids}]")
    }
}

impl From<Vec<Ulid>> for Relation {
    fn from(vec: Vec<Ulid>) -> Self {
        Self(vec)
    }
}

impl From<Ulid> for Relation {
    fn from(ulid: Ulid) -> Self {
        Self(vec![ulid])
    }
}

impl Filterable for Relation {
    fn as_text(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Inner<Self> for Relation {
    fn inner(&self) -> &Self {
        self
    }

    fn into_inner(self) -> Self {
        self
    }
}

impl Orderable for Relation {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl SanitizeManual for Relation {}

impl SanitizeAuto for Relation {}

impl ValidateManual for Relation {}

impl ValidateAuto for Relation {}

impl Visitable for Relation {}
