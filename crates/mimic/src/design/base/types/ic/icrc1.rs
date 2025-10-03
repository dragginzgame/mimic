use crate::design::prelude::*;

///
/// Icrc1 Payment
///

#[record(fields(
    field(ident = "recipient", value(item(prim = "Principal"))),
    field(ident = "tokens", value(item(is = "Tokens")))
))]
pub struct Payment {}

///
/// Icrc1 Tokens
///

#[record(fields(
    field(ident = "ledger_canister", value(item(prim = "Principal"))),
    field(ident = "tokens", value(item(prim = "Nat64")))
))]
pub struct Tokens {}
