pub mod icrc1;
pub mod icrc3;

use crate::design::prelude::*;

///
/// Memo
///

#[newtype(primitive = "Blob", item(prim = "Blob"))]
pub struct Memo {}

///
/// Payment
///

#[record(
    field(name = "recipient", value(item(prim = "Principal"))),
    field(name = "tokens", value(item(is = "Tokens")))
)]
pub struct Payment {}

///
/// Tokens
/// always denominated in e8s
///

#[newtype(primitive = "Nat64", item(prim = "Nat64"))]
pub struct Tokens {}
