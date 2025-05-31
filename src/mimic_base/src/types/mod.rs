pub mod ascii;
pub mod bytes;
pub mod color;
pub mod hash;
pub mod ic;
pub mod lang;
pub mod math;
pub mod text;
pub mod time;
pub mod web;

use crate::prelude::*;

///
/// Bool
///

#[primitive(variant = "Bool", path = "bool")]
pub struct Bool {}

///
/// Blob
///

#[primitive(variant = "Blob", path = "mimic::types::prim::Blob")]
pub struct Blob {}

///
/// Decimal
///

#[primitive(variant = "Decimal", path = "mimic::types::prim::Decimal")]
pub struct Decimal {}

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

#[primitive(variant = "Int", path = "mimic::types::prim::Int")]
pub struct Int {}

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

#[primitive(variant = "Nat", path = "mimic::types::prim::Nat")]
pub struct Nat {}

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

#[primitive(variant = "Principal", path = "mimic::types::prim::Principal")]
pub struct Principal {}

///
/// Relation
///

#[primitive(variant = "Relation", path = "mimic::types::prim::Relation")]
pub struct Relation {}

///
/// RelationSet
///

#[primitive(variant = "RelationSet", path = "mimic::types::prim::RelationSet")]
pub struct RelationSet {}

///
/// Text
///

#[primitive(variant = "Text", path = "::std::string::String")]
pub struct Text {}

///
/// Ulid
///

#[primitive(variant = "Ulid", path = "mimic::types::prim::Ulid")]
pub struct Ulid {}

///
/// Unit
///

#[primitive(variant = "Unit", path = "mimic::types::prim::Unit")]
pub struct Unit {}
