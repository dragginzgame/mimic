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

///
/// Float64
///

#[newtype(primitive = "Float64", item(prim = "Float64"))]
pub struct Float64 {}
