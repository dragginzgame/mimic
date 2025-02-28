pub mod ascii;
pub mod bytes;
pub mod color;
pub mod lang;
pub mod math;
pub mod prim;
pub mod text;
pub mod time;

use crate::orm::{base::types, prelude::*};

pub use prim::ulid::UlidSet;

///
/// Unit
///

#[primitive(variant = "Unit", path = "types::prim::Unit")]
pub struct Unit {}

impl Path for Unit {
    const IDENT: &'static str = "Unit";
    const PATH: &'static str = "mimic::orm::base::types::Unit";
}

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
/// F32
///

#[primitive(variant = "F32", path = "f32")]
pub struct F32 {}

///
/// F64
///

#[primitive(variant = "F64", path = "f64")]
pub struct F64 {}

///
/// I8
///

#[primitive(variant = "I8", path = "i8")]
pub struct I8 {}

///
/// I16
///

#[primitive(variant = "I16", path = "i16")]
pub struct I16 {}

///
/// I32
///

#[primitive(variant = "I32", path = "i32")]
pub struct I32 {}

///
/// I64
///

#[primitive(variant = "I64", path = "i64")]
pub struct I64 {}

///
/// I128
///

#[primitive(variant = "I128", path = "i128")]
pub struct I128 {}

///
/// Isize
///

#[primitive(variant = "Isize", path = "isize")]
pub struct Isize {}

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
/// U8
///

#[primitive(variant = "U8", path = "u8")]
pub struct U8 {}

///
/// U16
///

#[primitive(variant = "U16", path = "u16")]
pub struct U16 {}

///
/// U32
///

#[primitive(variant = "U32", path = "u32")]
pub struct U32 {}

///
/// U64
///

#[primitive(variant = "U64", path = "u64")]
pub struct U64 {}

///
/// U128
///

#[primitive(variant = "U128", path = "u128")]
pub struct U128 {}

///
/// Usize
///

#[primitive(variant = "Usize", path = "usize")]
pub struct Usize {}
