pub(crate) mod prelude {
    pub use mimic::design::{
        base::{types, validator},
        prelude::*,
    };
}
pub use prelude::*;

///
/// Float64
///

#[newtype(
    primitive = "Float64",
    item(prim = "Float64"),
    ty(validator(path = "validator::number::Ltoe", args(5.0)))
)]
pub struct Float64 {}
