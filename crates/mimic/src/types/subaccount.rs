use crate::{
    ops::traits::{
        FieldOrderable, FieldQueryable, FieldSortKey, Inner, ValidateAuto, ValidateCustom,
        Visitable,
    },
    types::Principal,
};
use derive_more::{Deref, DerefMut};
use icu::{
    ic::{
        candid::CandidType, ledger_types::Subaccount as WrappedSubaccount,
        principal::Principal as WrappedPrincipal,
    },
    impl_storable_bounded,
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

///
/// Subaccount
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
pub struct Subaccount(WrappedSubaccount);

impl Subaccount {
    #[must_use]
    pub fn to_bytes(self) -> [u8; 32] {
        self.0.0
    }

    #[must_use]
    pub fn from_u128s(a: u128, b: u128) -> Self {
        let mut bytes = [0u8; 32];
        bytes[..16].copy_from_slice(&a.to_be_bytes());
        bytes[16..].copy_from_slice(&b.to_be_bytes());

        Self(WrappedSubaccount(bytes))
    }
}

impl Default for Subaccount {
    fn default() -> Self {
        Self(WrappedSubaccount([0u8; 32]))
    }
}

impl Display for Subaccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl FieldOrderable for Subaccount {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldSortKey for Subaccount {
    fn to_sort_key_part(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl FieldQueryable for Subaccount {
    fn to_query_value(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl From<Principal> for Subaccount {
    fn from(principal: Principal) -> Self {
        Self((*principal).into())
    }
}

impl From<WrappedPrincipal> for Subaccount {
    fn from(principal: WrappedPrincipal) -> Self {
        Self(principal.into())
    }
}

impl From<Subaccount> for WrappedSubaccount {
    fn from(sub: Subaccount) -> Self {
        sub.0
    }
}

impl From<WrappedSubaccount> for Subaccount {
    fn from(wrap: WrappedSubaccount) -> Self {
        Subaccount(wrap)
    }
}

impl Inner for Subaccount {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        *self
    }

    fn into_inner(self) -> Self::Primitive {
        self
    }
}

impl PartialEq<Subaccount> for WrappedSubaccount {
    fn eq(&self, other: &Subaccount) -> bool {
        self == &other.0
    }
}

impl PartialEq<WrappedSubaccount> for Subaccount {
    fn eq(&self, other: &WrappedSubaccount) -> bool {
        &self.0 == other
    }
}

impl_storable_bounded!(Subaccount, 32, true);

impl ValidateAuto for Subaccount {}

impl ValidateCustom for Subaccount {}

impl Visitable for Subaccount {}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn subaccount_is_32_bytes() {
        assert_eq!(mem::size_of::<Subaccount>(), 32);
    }
}
