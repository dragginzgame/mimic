pub(crate) mod prelude {
    pub use mimic::design::{
        base::{types, validator},
        prelude::*,
    };
}
pub use prelude::*;

///
/// Float32
///

#[newtype(primitive = "Float32", item(prim = "Float32"))]
pub struct Float32 {}

#[newtype(primitive = "Float32", item(is = "Float32"))]
pub struct Float32W {}

#[newtype(primitive = "Float32", item(is = "Float32W"))]
pub struct Float32WW {}

///
/// Int
///

#[newtype(primitive = "Int", item(prim = "Int"))]
pub struct Int {}

///
/// Nat
///

#[newtype(primitive = "Nat", item(prim = "Nat"))]
pub struct Nat {}

///
/// Nat32
///

#[newtype(primitive = "Nat32", item(prim = "Nat32"))]
pub struct Nat32 {}

#[newtype(primitive = "Nat32", item(is = "Nat32"))]
pub struct Nat32W {}

#[newtype(primitive = "Nat32", item(is = "Nat32W"))]
pub struct Nat32WW {}

///
/// Principal
///

#[newtype(primitive = "Principal", item(prim = "Principal"))]
pub struct Principal {}

///
/// Subaccount
///

#[newtype(primitive = "Subaccount", item(prim = "Subaccount"))]
pub struct Subaccount {}

///
/// Ulid
///

#[newtype(primitive = "Ulid", item(prim = "Ulid"))]
pub struct Ulid {}

///
/// Unit
///

#[newtype(primitive = "Unit", item(prim = "Unit"))]
pub struct Unit {}
