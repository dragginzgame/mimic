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
/// wrapped primitive
///

#[newtype(primitive = "Float64", item(prim = "Float64"))]
pub struct Float64 {}

///
/// WrapFloat64
/// double wrapped primitive
///

#[newtype(item(is = "Float64"))]
pub struct WrapFloat64 {}
