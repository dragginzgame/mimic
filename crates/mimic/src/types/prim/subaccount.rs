use crate::traits::{FormatSortKey, Inner, Orderable, ValidateAuto, ValidateCustom, Visitable};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use icu::impl_storable_bounded;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{self},
};

///
/// Subaccount
///

#[derive(
    CandidType,
    Clone,
    Copy,
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
pub struct Subaccount([u8; 32]);

impl Subaccount {
    #[must_use]
    pub fn from_u128s(a: u128, b: u128) -> Self {
        let mut bytes = [0u8; 32];
        bytes[..16].copy_from_slice(&a.to_be_bytes());
        bytes[16..].copy_from_slice(&b.to_be_bytes());

        Subaccount(bytes)
    }
}

impl AsRef<[u8]> for Subaccount {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for Subaccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl From<Subaccount> for [u8; 32] {
    fn from(sub: Subaccount) -> Self {
        sub.0
    }
}

impl From<[u8; 32]> for Subaccount {
    fn from(bytes: [u8; 32]) -> Self {
        Subaccount(bytes)
    }
}

impl FormatSortKey for Subaccount {
    fn format_sort_key(&self) -> Option<String> {
        Some(self.to_string())
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

impl Orderable for Subaccount {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl PartialEq<Subaccount> for [u8; 32] {
    fn eq(&self, other: &Subaccount) -> bool {
        self == &other.0
    }
}

impl PartialEq<[u8; 32]> for Subaccount {
    fn eq(&self, other: &[u8; 32]) -> bool {
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
