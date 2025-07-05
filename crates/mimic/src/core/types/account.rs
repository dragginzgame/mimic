use crate::core::{
    traits::{
        FieldSearchable, FieldSortable, FieldValue, Inner, TypeView, ValidateAuto, ValidateCustom,
        Visitable,
    },
    types::{Principal, Subaccount},
    value::Value,
};
use derive_more::{Deref, DerefMut, Display};
use icu::{
    ic::{
        candid::CandidType, icrc_ledger_types::icrc1::account::Account as WrappedAccount,
        principal::Principal as WrappedPrincipal,
    },
    impl_storable_bounded,
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

///
/// Account
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
pub struct Account(WrappedAccount);

impl Account {
    pub fn new(owner: Principal, subaccount: Option<Subaccount>) -> Self {
        Self(WrappedAccount {
            owner: *owner,
            subaccount: subaccount.map(Subaccount::to_bytes),
        })
    }
}

impl FieldSearchable for Account {
    fn to_searchable_string(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl FieldSortable for Account {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldValue for Account {
    fn to_value(&self) -> Value {
        Value::Text(self.to_string())
    }
}

impl From<Principal> for Account {
    fn from(principal: Principal) -> Self {
        Self((*principal).into())
    }
}

impl From<WrappedPrincipal> for Account {
    fn from(principal: WrappedPrincipal) -> Self {
        Self(principal.into())
    }
}

impl From<Account> for WrappedAccount {
    fn from(account: Account) -> Self {
        account.0
    }
}

impl From<WrappedAccount> for Account {
    fn from(wrap: WrappedAccount) -> Self {
        Self(wrap)
    }
}

impl Inner for Account {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        *self
    }

    fn into_inner(self) -> Self::Primitive {
        self
    }
}

impl PartialEq<Account> for WrappedAccount {
    fn eq(&self, other: &Account) -> bool {
        self == &other.0
    }
}

impl PartialEq<WrappedAccount> for Account {
    fn eq(&self, other: &WrappedAccount) -> bool {
        &self.0 == other
    }
}

impl_storable_bounded!(Account, 63, true);

impl TypeView for Account {
    type View = WrappedAccount;

    fn to_view(&self) -> Self::View {
        self.0
    }

    fn from_view(view: Self::View) -> Self {
        Self(view)
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
    use std::mem;

    #[test]
    fn account_is_63_bytes() {
        assert_eq!(mem::size_of::<Account>(), 63);
    }

    #[test]
    fn account_roundtrip() {
        let p = Principal::anonymous();
        let s = Some(Subaccount::new([1; 32]));

        let a = Account::new(p, s);
        let b = WrappedAccount::from(a);
        let c = Account::from(b);

        assert_eq!(a, c);
    }
}
