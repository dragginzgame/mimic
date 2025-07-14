use crate::{
    node::{ValidateNode, VisitableNode},
    visit::Visitor,
};
use derive_more::{Deref, Display};
use serde::Serialize;

///
/// Arg
///

#[derive(Clone, Debug, Display, Serialize)]
pub enum Arg {
    Bool(bool),
    Char(char),
    Number(ArgNumber),
    Path(&'static str),
    String(&'static str),
}

impl ValidateNode for Arg {}

impl VisitableNode for Arg {
    fn route_key(&self) -> String {
        format!("arg ({self})")
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        if let Self::Number(node) = self {
            node.accept(v);
        }
    }
}

///
/// Args
///

#[derive(Clone, Debug, Deref, Serialize)]
pub struct Args(pub &'static [Arg]);

impl ValidateNode for Args {}

///
/// ArgNumber
///

#[derive(Clone, Debug, Display, Serialize)]
pub enum ArgNumber {
    Decimal(String),
    Float32(f32),
    Float64(f64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Nat8(u8),
    Nat16(u16),
    Nat32(u32),
    Nat64(u64),
}

impl ValidateNode for ArgNumber {}

impl VisitableNode for ArgNumber {}
