pub mod icrc1;
pub mod icrc3;

use crate::design::prelude::*;

///
/// Account
///

#[record(fields(
    field(name = "owner", value(item(prim = "Principal"))),
    field(name = "subaccount", value(opt, item(prim = "Subaccount")))
))]
pub struct Account {}

impl Account {
    pub fn new<P: Into<Principal>, S: Into<Subaccount>>(owner: P, subaccount: Option<S>) -> Self {
        Self {
            owner: owner.into(),
            subaccount: subaccount.map(Into::into),
        }
    }
}

impl<P: Into<Principal>> From<P> for Account {
    fn from(p: P) -> Self {
        Self {
            owner: p.into(),
            ..Default::default()
        }
    }
}

///
/// Memo
///

#[newtype(primitive = "Blob", item(prim = "Blob"))]
pub struct Memo {}

///
/// Icrc1 PaymentList
///

#[list(item(is = "Payment"))]
pub struct PaymentList {}

///
/// Payment
///

#[record(fields(
    field(name = "recipient", value(item(prim = "Principal"))),
    field(name = "tokens", value(item(is = "Tokens")))
))]
pub struct Payment {}

///
/// Tokens
/// always denominated in e8s
///

#[newtype(primitive = "Nat64", item(prim = "Nat64"))]
pub struct Tokens {}
