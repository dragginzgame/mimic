pub mod ascii;
pub mod bytes;
pub mod color;
pub mod lang;
pub mod math;
pub mod orm;
pub mod test;
pub mod text;
pub mod time;

// re-exports and aliases
pub type Todo = Bool;

use mimic::orm::traits::Path;

///
/// Blob
///

#[primitive(ty = "mimic::orm::types::Blob")]
pub struct Blob {}

///
/// Bool
///

#[primitive(ty = bool)]
pub struct Bool {}

///
/// Decimal
///

#[primitive(ty = "mimic::orm::types::Decimal")]
pub struct Decimal {}

///
/// F32
///

#[primitive(ty = f32)]
pub struct F32 {}

///
/// F64
///

#[primitive(ty = f64)]
pub struct F64 {}

///
/// I8
///

#[primitive(ty = i8)]
pub struct I8 {}

///
/// I16
///

#[primitive(ty = i16)]
pub struct I16 {}

///
/// I32
///

#[primitive(ty = i32)]
pub struct I32 {}

///
/// I64
///

#[primitive(ty = i64)]
pub struct I64 {}

///
/// I128
///

#[primitive(ty = i128)]
pub struct I128 {}

///
/// Principal
///

#[primitive(ty = "mimic::orm::types::Principal")]
pub struct Principal {}

///
/// String
///

#[primitive(ty = ::std::string::String)]
pub struct String {}

///
/// Timestamp
///

#[primitive(ty = "mimic::orm::types::Timestamp")]
pub struct Timestamp {}

///
/// Ulid
///

#[primitive(ty = "mimic::orm::types::Ulid")]
pub struct Ulid {}

///
/// U8
///

#[primitive(ty = u8)]
pub struct U8 {}

///
/// U16
///

#[primitive(ty = u16)]
pub struct U16 {}

///
/// U32
///

#[primitive(ty = u32)]
pub struct U32 {}

///
/// U64
///

#[primitive(ty = u64)]
pub struct U64 {}

///
/// U128
///

#[primitive(ty = u128)]
pub struct U128 {}
