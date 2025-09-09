use crate::design::prelude::*;

///
/// Icrc1 Payment
///

#[record(fields(
    field(name = "recipient", value(item(prim = "Principal"))),
    field(name = "tokens", value(item(is = "Tokens")))
))]
pub struct Payment {}

///
/// Icrc1 Tokens
///

#[record(fields(
    field(name = "ledger_canister", value(item(prim = "Principal"))),
    field(name = "tokens", value(item(prim = "Nat64")))
))]
pub struct Tokens {}
