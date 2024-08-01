use candid::{types::principal::PrincipalError, CandidType, Principal as WrappedPrincipal};
use derive_more::{Deref, DerefMut};
use ic::api::caller;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{
    fmt::{self},
    str::FromStr,
};

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
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
        WrappedPrincipal::from_str(s).map(Principal)
    }
}
