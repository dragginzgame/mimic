pub mod icrc1;
pub mod icrc3;

use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// Account
///

#[record(fields(
    field(name = "owner", value(item(is = "types::Principal"))),
    field(name = "subaccount", value(opt, item(is = "Subaccount"))),
))]
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

#[record(fields(
    field(name = "recipient", value(item(is = "Principal"))),
    field(name = "tokens", value(item(is = "Tokens"))),
))]
pub struct Payment {}

///
/// Tokens
///

#[newtype(primitive = "Nat64", item(is = "types::Nat64"))]
pub struct Tokens {}
