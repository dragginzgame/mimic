use derive_more::{Deref, DerefMut, FromStr};
use mimic::orm::prelude::*;
use mimic::{
    orm::{
        collections::HashSet,
        traits::{
            EntityFixture, Filterable, Orderable, Path, PrimaryKey, SanitizeAuto, Validate,
            ValidateAuto,
        },
    },
    types::{ErrorVec, Ulid as WrappedUlid},
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{cmp::Ordering, fmt};

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Ulid { source: mimic::types::ulid::Error },
}

///
/// Ulid
///

#[derive(
    CandidType,
    Clone,
    Copy,
    Debug,
    Deref,
    DerefMut,
    Eq,
    FromStr,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct Ulid(WrappedUlid);

impl Ulid {
    /// nil
    #[must_use]
    pub const fn nil() -> Self {
        Self(WrappedUlid::nil())
    }

    /// from_string
    pub fn from_string(encoded: &str) -> Result<Self, Error> {
        let ulid = WrappedUlid::from_string(encoded)?;

        Ok(Self(ulid))
    }

    /// generate
    /// Generate a ULID string with the current timestamp and a random value
    #[must_use]
    pub fn generate() -> Self {
        Self(WrappedUlid::generate())
    }

    // from_string_digest
    #[must_use]
    pub fn from_string_digest(name: &str) -> Self {
        Self(WrappedUlid::from_string_digest(name))
    }
}

impl Default for Ulid {
    fn default() -> Self {
        Self(WrappedUlid::nil())
    }
}

impl fmt::Display for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Filterable for Ulid {
    fn as_text(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl<T> From<T> for Ulid
where
    T: EntityFixture,
{
    fn from(entity: T) -> Self {
        Self(entity.ulid())
    }
}

impl Orderable for Ulid {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl PrimaryKey for Ulid {
    fn on_create(&self) -> Self {
        Self::generate()
    }

    fn format(&self) -> String {
        self.0.to_string()
    }
}

impl SanitizeManual for Ulid {}

impl SanitizeAuto for Ulid {}

impl ValidateManual for Ulid {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        if self.is_nil() {
            Err(mimic::types::ulid::Error::Nil.into())
        } else {
            Ok(())
        }
    }
}

impl ValidateAuto for Ulid {}

impl Visitable for Ulid {}
