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
            .map(|ulid| ulid.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{}]", ulids)
    }
}

impl From<Vec<Ulid>> for Relation {
    fn from(vec: Vec<Ulid>) -> Self {
        Relation(vec)
    }
}

impl From<Ulid> for Relation {
    fn from(ulid: Ulid) -> Self {
        Relation(vec![ulid])
    }
}

impl FromStr for Relation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        // Ensure the string starts with '[' and ends with ']'
        if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
            return Err("Relation string must start with '[' and end with ']'".to_string());
        }

        // Remove the enclosing brackets
        let inner = &trimmed[1..trimmed.len() - 1];

        // Split by commas and parse each part as a ULID
        let ulids = inner
            .split(',')
            .map(|part| {
                let part = part.trim(); // Trim whitespace around each ULID
                Ulid::from_str(part).map_err(|e| format!("Invalid ULID: {}: {}", part, e))
            })
            .collect::<Result<Vec<Ulid>, String>>()?;

        Ok(Relation(ulids))
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

impl PrimaryKey for Relation {
    fn on_create(&self) -> Self {
        self.clone()
    }

    fn format(&self) -> String {
        self.to_string()
    }
}

impl SanitizeManual for Relation {}

impl SanitizeAuto for Relation {}

impl ValidateManual for Relation {}

impl ValidateAuto for Relation {}

impl Visitable for Relation {}
