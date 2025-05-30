use crate::{base::types, prelude::*};

///
/// Icrc1 Payment
///

#[record(
    field(name = "recipient", value(item(is = "Principal"))),
    field(name = "tokens", value(item(is = "Tokens")))
)]
pub struct Payment {}

///
/// Icrc1 Tokens
///

#[record(
    field(name = "ledger_canister", value(item(is = "types::Principal"))),
    field(name = "tokens", value(item(is = "Nat64")))
)]
pub struct Tokens {}
