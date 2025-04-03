use crate::orm::{base::types, prelude::*};

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
/// Value
/// Generic value in accordance with ICRC-3
///

#[enum_(
    variant(name = "Array", value(many, item(is = "Value"))),
    variant(name = "Blob", value(item(is = "types::Blob"))),
    variant(name = "Int", value(item(is = "types::I64"))),
    variant(name = "Map", value(item(is = "Map"))),
    variant(name = "Nat", value(item(is = "types::U64"))),
    variant(name = "Text", value(item(is = "types::String")))
)]
pub struct Value {}

///
/// Value Map
///

#[map(key(is = "types::String"), value(item(is = "Value")))]
pub struct Map {}
