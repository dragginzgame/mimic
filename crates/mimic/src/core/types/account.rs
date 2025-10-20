use crate::core::{
    Value,
    traits::{
        FieldValue, Inner, SanitizeAuto, SanitizeCustom, TypeView, ValidateAuto, ValidateCustom,
        Visitable,
    },
};
use candid::CandidType;
use canic::{
    impl_storable_bounded,
    types::{Account as WrappedAccount, Principal, Subaccount},
};
use derive_more::{Deref, DerefMut, Display};
use serde::{Deserialize, Serialize};

///
/// Account
///

#[derive(
    CandidType,
    Debug,
    Deref,
    DerefMut,
    Display,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
    Ord,
    Serialize,
    Deserialize,
    PartialOrd,
)]
pub struct Account(pub WrappedAccount);

impl Account {
    // this is using the icrc-ledger-types crate.  We could get it down to 62 but for
    // the moment I don't want to try that
    pub const STORABLE_MAX_SIZE: u32 = 128;

    pub fn new<P: Into<Principal>, S: Into<Subaccount>>(owner: P, subaccount: Option<S>) -> Self {
        Self(WrappedAccount {
            owner: owner.into(),
            subaccount: subaccount.map(Into::into),
        })
    }

    /// Convert the account into a deterministic IC-style byte representation.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let principal_bytes = self.owner.as_slice();
        let subaccount_bytes = self.subaccount.unwrap_or_default();
        let mut out = Vec::with_capacity(1 + principal_bytes.len() + 32);

        // Encode principal length (so reversible)
        #[allow(clippy::cast_possible_truncation)]
        out.push(principal_bytes.len() as u8);
        out.extend_from_slice(principal_bytes);
        out.extend_from_slice(&subaccount_bytes);

        out
    }

    /// Construct the maximum possible account for storage sizing tests.
    #[must_use]
    pub const fn max_storable() -> Self {
        // 29 bytes of non-zero data to simulate a maximal Principal
        let principal_bytes = [0xFFu8; 29];
        let owner = Principal::from_slice(&principal_bytes);
        let subaccount = [0xFFu8; 32];

        Self(WrappedAccount {
            owner,
            subaccount: Some(subaccount),
        })
    }
}

impl Default for Account {
    fn default() -> Self {
        Self(Principal::from_slice(&[]).into())
    }
}

impl FieldValue for Account {
    fn to_value(&self) -> Value {
        Value::Account(*self)
    }
}

impl From<Principal> for Account {
    fn from(owner: Principal) -> Self {
        Self(owner.into())
    }
}

impl Inner<Self> for Account {
    fn inner(&self) -> &Self {
        self
    }

    fn into_inner(self) -> Self {
        self
    }
}

impl SanitizeAuto for Account {}

impl SanitizeCustom for Account {}

impl_storable_bounded!(Account, Account::STORABLE_MAX_SIZE, true);

impl TypeView for Account {
    type View = Self;

    fn to_view(&self) -> Self::View {
        *self
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

impl ValidateAuto for Account {}

impl ValidateCustom for Account {}

impl Visitable for Account {}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::traits::Storable;
    use candid::Principal;

    fn principal() -> Principal {
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
    }

    #[test]
    fn max_size_is_bounded() {
        let account = Account::max_storable();
        let size = Storable::to_bytes(&account).len();

        assert!(
            size <= Account::STORABLE_MAX_SIZE as usize,
            "serialized Account too large: got {size} bytes, expected <= {}",
            Account::STORABLE_MAX_SIZE
        );
    }

    #[test]
    fn to_bytes_is_deterministic() {
        let acc1 = Account::new(principal(), None::<Subaccount>);
        let acc2 = Account::new(principal(), None::<Subaccount>);
        assert_eq!(acc1.to_bytes(), acc2.to_bytes());
    }

    #[test]
    fn to_bytes_length_is_consistent() {
        let acc = Account::new(principal(), Some([1u8; 32]));
        let bytes = acc.to_bytes();
        assert_eq!(bytes[0] as usize + 1 + 32, bytes.len());
    }

    #[test]
    fn from_principal_creates_account_with_empty_subaccount() {
        let p = principal();
        let acc = Account::from(p);
        assert_eq!(acc.owner, p);
        assert!(acc.subaccount.is_none());
    }

    #[test]
    fn default_account_is_empty_principal_and_none_subaccount() {
        let acc = Account::default();
        assert!(acc.owner.as_slice().is_empty());
        assert!(acc.subaccount.is_none());
    }

    #[test]
    fn new_with_subaccount_sets_fields_correctly() {
        let sub: Subaccount = [42u8; 32];
        let acc = Account::new(principal(), Some(sub));
        assert_eq!(acc.owner, principal());
        assert_eq!(acc.subaccount, Some(sub));
    }

    #[test]
    fn to_bytes_produces_expected_layout() {
        let p = principal();
        let acc = Account::new(p, Some([0xAAu8; 32]));
        let bytes = acc.to_bytes();

        let len = bytes[0] as usize;
        let principal_part = &bytes[1..=len];
        let subaccount_part = &bytes[1 + len..];

        assert_eq!(principal_part, p.as_slice());
        assert_eq!(subaccount_part, &[0xAAu8; 32]);
    }

    #[test]
    fn to_bytes_with_none_subaccount_encodes_zero_bytes() {
        let p = principal();
        let acc = Account::new(p, None::<Subaccount>);
        let bytes = acc.to_bytes();
        let len = bytes[0] as usize;
        let subaccount_part = &bytes[1 + len..];
        assert!(subaccount_part.iter().all(|&b| b == 0));
    }
}
