use crate::{
    ic::{api::caller, structures::storable::Bound},
    orm::{
        prelude::*,
        traits::{
            Filterable, Inner, Orderable, PrimaryKey, SanitizeAuto, SanitizeManual, Storable,
            ValidateAuto, ValidateManual, Visitable,
        },
    },
};
use candid::{types::principal::PrincipalError, Principal as WrappedPrincipal};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::{self},
    str::FromStr,
};

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("principal is empty"))]
    EmptyPrincipal,
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
        WrappedPrincipal::from_str(s).map(Self)
    }
}

impl Inner<Self> for Principal {
    fn inner(&self) -> &Self {
        self
    }
}

impl Orderable for Principal {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl PrimaryKey for Principal {
    fn on_create(&self) -> Self {
        *self
    }

    fn format(&self) -> String {
        self.0.to_string()
    }
}

impl SanitizeManual for Principal {}

impl SanitizeAuto for Principal {}

impl Storable for Principal {
    fn to_bytes(&self) -> Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(WrappedPrincipal::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 32,
        is_fixed_size: true,
    };
}

impl ValidateManual for Principal {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        if self.0.as_slice().is_empty() {
            Err(Error::EmptyPrincipal.into())
        } else {
            Ok(())
        }
    }
}

impl ValidateAuto for Principal {}

impl Visitable for Principal {}
