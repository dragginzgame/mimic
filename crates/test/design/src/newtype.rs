pub(crate) mod prelude {
    pub use mimic::design::{
        base::{types, validator},
        prelude::*,
    };
}
pub use prelude::*;

///
/// Decimal
///

#[newtype(
    primitive = "Decimal",
    item(prim = "Decimal"),
    ty(validator(path = "validator::number::Ltoe", args(5.0)))
)]
pub struct Decimal {}

///
/// Float32
///

#[newtype(
    primitive = "Float32",
    item(prim = "Float32"),
    ty(validator(path = "validator::number::Ltoe", args(5.0)))
)]
pub struct Float32 {}

///
/// Float64
///

#[newtype(
    primitive = "Float64",
    item(prim = "Float64"),
    ty(validator(path = "validator::number::Ltoe", args(5.0)))
)]
pub struct Float64 {}
