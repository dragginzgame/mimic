use crate::{
    ic::{api::caller, structures::storable::Bound},
    impl_storable_bounded,
    orm::{
        prelude::*,
        traits::{
            Filterable, Inner, Orderable, SortKey, Storable, ValidateAuto, ValidateManual,
            Visitable,
        },
    },
    Error, ThisError,
};
use candid::{CandidType, Principal as WrappedPrincipal};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::{self},
    str::FromStr,
};

///
/// PrincipalError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum PrincipalError {
    #[error("principal is empty")]
    EmptyPrincipal,

    #[error("{0}")]
    Wrapped(String),
}

///
/// Principal
///

#[derive(
    CandidType,
    Clone,
    Copy,
    Debug,
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
pub struct Principal(WrappedPrincipal);

impl Principal {
    #[must_use]
    pub fn caller() -> Self {
        Self(caller())
    }
}

impl Default for Principal {
    fn default() -> Self {
        Self(WrappedPrincipal::from_slice(&[]))
    }
}

impl fmt::Display for Principal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Filterable for Principal {
    fn as_text(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl From<WrappedPrincipal> for Principal {
    fn from(principal: WrappedPrincipal) -> Self {
        Self(principal)
    }
}

impl From<Principal> for WrappedPrincipal {
    fn from(p: Principal) -> Self {
        *p
    }
}

impl FromStr for Principal {
    type Err = PrincipalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let this = WrappedPrincipal::from_str(s)
            .map(Self)
            .map_err(|e| PrincipalError::Wrapped(e.to_string()))?;

        Ok(this)
    }
}

impl Inner<Self> for Principal {
    fn inner(&self) -> &Self {
        self
    }

    fn into_inner(self) -> Self {
        self
    }
}

impl Orderable for Principal {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl SortKey for Principal {}

impl_storable_bounded!(Principal, 32, false);

impl ValidateManual for Principal {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        if self.0.as_slice().is_empty() {
            Err(PrincipalError::EmptyPrincipal.into())
        } else {
            Ok(())
        }
    }
}

impl ValidateAuto for Principal {}

impl Visitable for Principal {}
