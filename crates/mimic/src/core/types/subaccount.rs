use crate::core::{
    traits::{
        FieldValue, SanitizeAuto, SanitizeCustom, TypeView, ValidateAuto, ValidateCustom, Visitable,
    },
    types::{Principal, Ulid},
    value::Value,
};
use candid::CandidType;
use canic::{
    impl_storable_bounded,
    types::{Principal as WrappedPrincipal, Subaccount as WrappedSubaccount},
    utils::rand::next_u128,
};
use derive_more::{Deref, DerefMut};
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
    pub const fn to_array(&self) -> [u8; 32] {
        self.0.0
    }

    #[must_use]
    pub const fn from_array(array: [u8; 32]) -> Self {
        Self(WrappedSubaccount(array))
    }

    #[must_use]
    pub fn from_ulid(ulid: Ulid) -> Self {
        let mut bytes = [0u8; 32];
        bytes[16..].copy_from_slice(&ulid.to_bytes()); // right-align ULID

        Self::from_array(bytes)
    }

    #[must_use]
    pub fn to_ulid(&self) -> Ulid {
        let bytes = self.to_array();
        let ulid_bytes: [u8; 16] = bytes[16..].try_into().expect("slice has exactly 16 bytes");

        Ulid::from_bytes(ulid_bytes)
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

    #[test]
    fn round_trip_ulid_to_subaccount_and_back() {
        let ulid = Ulid::default();
        let sub = Subaccount::from_ulid(ulid);
        let ulid2 = sub.to_ulid();

        assert_eq!(ulid, ulid2);
    }

    #[test]
    fn different_ulids_produce_different_subaccounts() {
        let ulid1 = Ulid::generate();
        let ulid2 = Ulid::generate();
        assert_ne!(
            Subaccount::from_ulid(ulid1).to_array(),
            Subaccount::from_ulid(ulid2).to_array()
        );
    }
}
