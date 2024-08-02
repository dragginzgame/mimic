pub mod ascii;
pub mod bytes;
pub mod color;
pub mod lang;
pub mod math;
pub mod orm;
pub mod prim;
pub mod test;
pub mod text;
pub mod time;

pub use crate::prelude::*;
use crate::types;

// re-exports and aliases
pub type Todo = Bool;

///
/// Bool
///

#[primitive(ty = "Bool", path = "bool")]
pub struct Bool {}

///
/// Blob
///

#[primitive(ty = "Blob", path = "types::prim::Blob")]
pub struct Blob {}

impl Path for Blob {
    const IDENT: &'static str = "Blob";
    const PATH: &'static str = "base::types::Blob";
}

///
/// Decimal
///

#[primitive(ty = "Decimal", path = "types::prim::Decimal")]
pub struct Decimal {}

impl Path for Decimal {
    const IDENT: &'static str = "Decimal";
    const PATH: &'static str = "base::types::Decimal";
}

///
/// F32
///

#[primitive(ty = "F32", path = "f32")]
pub struct F32 {}

///
/// F64
///

#[primitive(ty = "F64", path = "f64")]
pub struct F64 {}

///
/// I8
///

#[primitive(ty = "I8", path = "i8")]
pub struct I8 {}

///
/// I16
///

#[primitive(ty = "I16", path = "i16")]
pub struct I16 {}

///
/// I32
///

#[primitive(ty = "I32", path = "i32")]
pub struct I32 {}

///
/// I64
///

#[primitive(ty = "I64", path = "i64")]
pub struct I64 {}

///
/// I128
///

#[primitive(ty = "I128", path = "i128")]
pub struct I128 {}

///
/// Principal
///

#[primitive(ty = "Principal", path = "types::prim::Principal")]
pub struct Principal {}

impl Path for Principal {
    const IDENT: &'static str = "Principal";
    const PATH: &'static str = "base::types::Principal";
}

///
/// String
///

#[primitive(ty = "String", path = "::std::string::String")]
pub struct String {}

///
/// Timestamp
///

#[primitive(ty = "Timestamp", path = "types::prim::Timestamp")]
pub struct Timestamp {}

impl Path for Timestamp {
    const IDENT: &'static str = "Timestamp";
    const PATH: &'static str = "base::types::Timestamp";
}

///
/// Ulid
///

#[primitive(ty = "Ulid", path = "types::prim::Ulid")]
pub struct Ulid {}

impl Path for Ulid {
    const IDENT: &'static str = "Ulid";
    const PATH: &'static str = "base::types::Ulid";
}

///
/// U8
///

#[primitive(ty = "U8", path = "u8")]
pub struct U8 {}

///
/// U16
///

#[primitive(ty = "U16", path = "u16")]
pub struct U16 {}

///
/// U32
///

#[primitive(ty = "U32", path = "u32")]
pub struct U32 {}

///
/// U64
///

#[primitive(ty = "U64", path = "u64")]
pub struct U64 {}

///
/// U128
///

#[primitive(ty = "U128", path = "u128")]
pub struct U128 {}
