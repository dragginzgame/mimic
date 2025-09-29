use crate::core::{
    traits::{
        FieldValue, SanitizeAuto, SanitizeCustom, TypeView, ValidateAuto, ValidateCustom, Visitable,
    },
    types::Principal,
    value::Value,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use icu::{
    impl_storable_bounded,
    types::{Principal as WrappedPrincipal, Subaccount as WrappedSubaccount},
    utils::rand::next_u128,
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

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
    pub const STORABLE_MAX_SIZE: u32 = 72;
    pub const MIN: Self = Self::from_array([0x00; 32]);
    pub const MAX: Self = Self::from_array([0xFF; 32]);

    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(WrappedSubaccount(bytes))
    }

    #[must_use]
    pub const fn from_array(array: [u8; 32]) -> Self {
        Self(WrappedSubaccount(array))
    }

    /// Generate a random subaccount using two 128-bit draws.
    #[must_use]
    pub fn random() -> Self {
        let hi = next_u128().to_le_bytes();
        let lo = next_u128().to_le_bytes();

        let mut bytes = [0u8; 32];
        bytes[..16].copy_from_slice(&hi);
        bytes[16..].copy_from_slice(&lo);

        Self::from_array(bytes)
    }

    #[must_use]
    pub const fn to_bytes(self) -> [u8; 32] {
        self.0.0
    }

    #[must_use]
    pub const fn max_storable() -> Self {
        Self(WrappedSubaccount([0xFF; 32]))
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
            write!(f, "{byte:02x}")?;
        }

        Ok(())
    }
}

impl FieldValue for Subaccount {
    fn to_value(&self) -> Value {
        Value::Text(self.to_string())
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
        Self(wrap)
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

impl SanitizeAuto for Subaccount {}

impl SanitizeCustom for Subaccount {}

impl_storable_bounded!(Subaccount, Subaccount::STORABLE_MAX_SIZE, true);

impl TypeView for Subaccount {
    type View = Self;

    fn to_view(&self) -> Self::View {
        *self
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

impl ValidateAuto for Subaccount {}

impl ValidateCustom for Subaccount {}

impl Visitable for Subaccount {}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::traits::Storable;

    #[test]
    fn subaccount_max_size_is_bounded() {
        let subaccount = Subaccount::max_storable();
        let size = Storable::to_bytes(&subaccount).len();

        assert!(size <= Subaccount::STORABLE_MAX_SIZE as usize);
    }

    #[test]
    fn generate_produces_valid_subaccount() {
        let sub = Subaccount::random();

        // Must always be exactly 32 bytes
        assert_eq!(sub.to_bytes().len(), 32);

        // Should not equal MIN or MAX every time
        assert_ne!(sub, Subaccount::MIN);
        assert_ne!(sub, Subaccount::MAX);
    }

    #[test]
    fn generate_produces_different_values() {
        let sub1 = Subaccount::random();
        let sub2 = Subaccount::random();

        // Extremely unlikely they’re equal in two calls
        assert_ne!(sub1, sub2);
    }

    #[test]
    fn generate_multiple_are_unique() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        for _ in 0..100 {
            let sub = Subaccount::random();
            assert!(set.insert(sub), "duplicate subaccount generated");
        }
    }

    #[test]
    fn display_hex_format_is_64_chars() {
        let sub = Subaccount::random();
        let hex = sub.to_string();

        // 32 bytes → 64 hex chars
        assert_eq!(hex.len(), 64);

        // Must be valid hex
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
