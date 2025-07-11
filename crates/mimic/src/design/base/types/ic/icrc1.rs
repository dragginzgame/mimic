use crate::design::prelude::*;

///
/// Icrc1 PaymentList
///

#[list(item(is = "Payment"))]
pub struct PaymentList {}

///
/// Icrc1 Payment
///

#[record(fields(
    field(name = "recipient", value(item(prim = "Principal"))),
    field(name = "tokens", value(item(is = "Tokens")))
))]
pub struct Payment {}

///
/// Icrc1 TokenList
///

#[list(item(is = "Payment"))]
pub struct TokenList {}

///
/// Icrc1 Tokens
///

#[record(fields(
    field(name = "ledger_canister", value(item(prim = "Principal"))),
    field(name = "tokens", value(item(prim = "Nat64")))
))]
pub struct Tokens {}
