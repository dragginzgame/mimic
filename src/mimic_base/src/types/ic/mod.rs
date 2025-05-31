pub mod icrc1;
pub mod icrc3;

use crate::prelude::*;

///
/// Account
///

#[record(
    field(name = "owner", value(item(is = "types::Principal"))),
    field(name = "subaccount", value(opt, item(is = "Subaccount")))
)]
pub struct Account {}

///
/// Subaccount
///

#[newtype(primitive = "Blob", item(is = "types::Blob"))]
pub struct Subaccount {}

///
/// Memo
///

#[newtype(primitive = "Blob", item(is = "types::Blob"))]
pub struct Memo {}

///
/// Payment
///

#[record(
    field(name = "recipient", value(item(is = "types::Principal"))),
    field(name = "tokens", value(item(is = "Tokens")))
)]
pub struct Payment {}

///
/// Tokens
/// always denominated in e8s
///

#[newtype(primitive = "Nat64", item(is = "types::Nat64"))]
pub struct Tokens {}
