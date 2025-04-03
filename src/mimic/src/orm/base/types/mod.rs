pub mod ascii;
pub mod bytes;
pub mod color;
pub mod icrc;
pub mod lang;
pub mod math;
pub mod prim;
pub mod text;
pub mod time;
pub mod web;

use crate::orm::{base::types, prelude::*};

pub use prim::ulid::UlidSet;

///
/// Bool
///

#[primitive(variant = "Bool", path = "bool")]
pub struct Bool {}

///
/// Blob
///

#[primitive(variant = "Blob", path = "types::prim::Blob")]
pub struct Blob {}

impl Path for Blob {
    const IDENT: &'static str = "Blob";
    const PATH: &'static str = "mimic::orm::base::types::Blob";
}

///
/// Decimal
///

#[primitive(variant = "Decimal", path = "types::prim::Decimal")]
pub struct Decimal {}

impl Path for Decimal {
    const IDENT: &'static str = "Decimal";
    const PATH: &'static str = "mimic::orm::base::types::Decimal";
}

///
/// Float32
///

#[primitive(variant = "Float32", path = "f32")]
pub struct Float32 {}

///
/// Float64
///

#[primitive(variant = "Float64", path = "f64")]
pub struct Float64 {}

///
/// Int
///

#[primitive(variant = "Int", path = "types::prim::Int")]
pub struct Int {}

impl Path for Int {
    const IDENT: &'static str = "Int";
    const PATH: &'static str = "mimic::orm::base::types::Int";
}

///
/// Int8
///

#[primitive(variant = "Int8", path = "i8")]
pub struct Int8 {}

///
/// Int16
///

#[primitive(variant = "Int16", path = "i16")]
pub struct Int16 {}

///
/// Int32
///

#[primitive(variant = "Int32", path = "i32")]
pub struct Int32 {}

///
/// Int64
///

#[primitive(variant = "Int64", path = "i64")]
pub struct Int64 {}

///
/// Int128
///

#[primitive(variant = "Int128", path = "i128")]
pub struct Int128 {}

///
/// Nat
///

#[primitive(variant = "Nat", path = "types::prim::Nat")]
pub struct Nat {}

impl Path for Nat {
    const IDENT: &'static str = "Nat";
    const PATH: &'static str = "mimic::orm::base::types::Nat";
}

///
/// Nat8
///

#[primitive(variant = "Nat8", path = "u8")]
pub struct Nat8 {}

///
/// Nat16
///

#[primitive(variant = "Nat16", path = "u16")]
pub struct Nat16 {}

///
/// Nat32
///

#[primitive(variant = "Nat32", path = "u32")]
pub struct Nat32 {}

///
/// Nat64
///

#[primitive(variant = "Nat64", path = "u64")]
pub struct Nat64 {}

///
/// Nat128
///

#[primitive(variant = "Nat128", path = "u128")]
pub struct Nat128 {}

///
/// Principal
///

#[primitive(variant = "Principal", path = "types::prim::Principal")]
pub struct Principal {}

impl Path for Principal {
    const IDENT: &'static str = "Principal";
    const PATH: &'static str = "mimic::orm::base::types::Principal";
}

///
/// String
///

#[primitive(variant = "String", path = "::std::string::String")]
pub struct String {}

///
/// Ulid
///

#[primitive(variant = "Ulid", path = "types::prim::Ulid")]
pub struct Ulid {}

impl Path for Ulid {
    const IDENT: &'static str = "Ulid";
    const PATH: &'static str = "mimic::orm::base::types::Ulid";
}

///
/// Unit
///

#[primitive(variant = "Unit", path = "types::prim::Unit")]
pub struct Unit {}

impl Path for Unit {
    const IDENT: &'static str = "Unit";
    const PATH: &'static str = "mimic::orm::base::types::Unit";
}
