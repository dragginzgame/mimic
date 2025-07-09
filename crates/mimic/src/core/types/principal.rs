use crate::{
    ThisError,
    common::error::ErrorTree,
    core::{
        traits::{
            FieldSearchable, FieldSortable, FieldValue, Inner, TypeView, ValidateAuto,
            ValidateCustom, Visitable,
        },
        value::Value,
    },
};
use derive_more::{Deref, DerefMut, Display};
use icu::{
    ic::{api::msg_caller, candid::CandidType, principal::Principal as WrappedPrincipal},
    impl_storable_bounded,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, str::FromStr};

///
/// PrincipalError
///

#[derive(Debug, ThisError)]
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
    Display,
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
    pub const STORABLE_MAX_SIZE: u32 = 32;
    pub const MIN: Self = Self::from_slice(&[0x00; 29]);
    pub const MAX: Self = Self::from_slice(&[0xFF; 29]);

    #[must_use]
    pub fn msg_caller() -> Self {
        Self(msg_caller())
    }

    #[must_use]
    pub const fn from_slice(slice: &[u8]) -> Self {
        Self(WrappedPrincipal::from_slice(slice))
    }

    #[must_use]
    pub const fn anonymous() -> Self {
        Self(WrappedPrincipal::anonymous())
    }

    #[must_use]
    pub fn max_self() -> Self {
        Self::from_slice(&[0xFF; 29])
    }
}

impl Default for Principal {
    fn default() -> Self {
        Self(WrappedPrincipal::from_slice(&[]))
    }
}

impl FieldSearchable for Principal {
    fn to_searchable_string(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl FieldSortable for Principal {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldValue for Principal {
    fn to_value(&self) -> Value {
        Value::Principal(*self)
    }
}

impl From<WrappedPrincipal> for Principal {
    fn from(p: WrappedPrincipal) -> Self {
        Self(p)
    }
}

impl From<&WrappedPrincipal> for Principal {
    fn from(p: &WrappedPrincipal) -> Self {
        Self(*p)
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

impl Inner for Principal {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        *self
    }

    fn into_inner(self) -> Self::Primitive {
        self
    }
}

impl PartialEq<WrappedPrincipal> for Principal {
    fn eq(&self, other: &WrappedPrincipal) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Principal> for WrappedPrincipal {
    fn eq(&self, other: &Principal) -> bool {
        *self == other.0
    }
}

impl_storable_bounded!(Principal, Principal::STORABLE_MAX_SIZE, true);

impl TypeView for Principal {
    type View = WrappedPrincipal;

    fn to_view(&self) -> Self::View {
        self.0
    }

    fn from_view(view: Self::View) -> Self {
        Self(view)
    }
}

impl ValidateAuto for Principal {
    fn validate_self(&self) -> Result<(), ErrorTree> {
        if self.0.as_slice().is_empty() {
            Err(PrincipalError::EmptyPrincipal.to_string().into())
        } else {
            Ok(())
        }
    }
}

impl ValidateCustom for Principal {}

impl Visitable for Principal {}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::traits::Storable;

    #[test]
    fn principal_max_size_is_bounded() {
        let principal = Principal::max_self();
        let size = Storable::to_bytes(&principal).len() as u32;

        println!("max serialized size = {size}");
        assert!(size <= Principal::STORABLE_MAX_SIZE);
    }
}
