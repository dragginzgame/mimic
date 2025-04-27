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

#[record(
    fields(field(name = "e8s", value(item(is = "Nat64")))),
    traits(add(Default))
)]
pub struct Tokens {}

impl From<u64> for Tokens {
    fn from(e8s: u64) -> Self {
        Self { e8s }
    }
}
