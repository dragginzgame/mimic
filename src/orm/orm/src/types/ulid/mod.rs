pub mod fixture;
pub mod generator;

use crate::{
    collections::HashSet,
    traits::{
        Filterable, Orderable, Path, PrimaryKey, Sanitize, SanitizeAuto, Validate, ValidateAuto,
        Visitable,
    },
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, FromStr};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{cmp::Ordering, fmt};
use types::{ErrorVec, Ulid as WrappedUlid};

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Ulid { source: types::ulid::Error },

    #[snafu(display("generator: {source}"))]
    Generator { source: generator::Error },
}

///
/// Ulid
///
/// an ORM type wrapper around types::Ulid
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
    Storable,
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
        generator::generate().unwrap()
    }

    /// fixture
    /// both paths are used in case two Fixture entities are trying to write to
    /// the same Entity and use the same ID
    #[must_use]
    pub fn fixture<T>(t: T) -> Self
    where
        T: Into<&'static str> + Path,
    {
        let digest = format!("{}-{}", T::PATH, t.into());
        Self::from_string_digest(&digest)
    }

    /// fixtures
    /// helper function for a many relation
    #[must_use]
    pub fn fixtures<T>(ts: Vec<T>) -> HashSet<Self>
    where
        T: Into<&'static str> + Path,
    {
        ts.into_iter()
            .map(|t| {
                let digest = format!("{}-{}", T::PATH, t.into());
                Self::from_string_digest(&digest)
            })
            .collect()
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

impl<T: Into<WrappedUlid>> From<T> for Ulid {
    fn from(t: T) -> Self {
        Self(t.into())
    }
}

impl Orderable for Ulid {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl Sanitize for Ulid {}

impl SanitizeAuto for Ulid {}

impl PrimaryKey for Ulid {
    fn on_create(&self) -> Self {
        Self::generate()
    }

    fn format(&self) -> String {
        self.0.to_string()
    }
}

impl Validate for Ulid {
    fn validate(&self) -> Result<(), ErrorVec> {
        if self.is_nil() {
            Err(types::ulid::Error::Nil.into())
        } else {
            Ok(())
        }
    }
}

impl ValidateAuto for Ulid {}

impl Visitable for Ulid {}
