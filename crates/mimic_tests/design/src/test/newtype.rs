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
/// Int128
///

#[newtype(primitive = "Int128", item(prim = "Int128"))]
pub struct Int128 {}

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
/// Nat64
///

#[newtype(primitive = "Nat64", item(prim = "Nat64"))]
pub struct Nat64 {}

///
/// Nat128
///

#[newtype(primitive = "Nat128", item(prim = "Nat128"))]
pub struct Nat128 {}

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

///
///  TESTS
///

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_references_with_primitive() {
        let tokens = Nat64::from(5_u64);

        assert_eq!(&tokens, 5_u64);
        assert_eq!(5_u64, &tokens);
        assert!(&tokens > 3_u64);
        assert!(3_u64 < &tokens);
        assert!(&tokens >= 5_u64);
        assert!(5_u64 <= &tokens);
    }
}
