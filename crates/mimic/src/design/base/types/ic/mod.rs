pub mod icrc1;
pub mod icrc3;

use crate::design::prelude::*;

///
/// Account
///

#[record(fields(
    field(name = "owner", value(item(prim = "Principal"))),
    field(name = "subaccount", value(item(prim = "Subaccount")))
))]
pub struct Account {}

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
