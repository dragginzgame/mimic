use crate::schema::{
    node::{ValidateNode, VisitableNode},
    visit::Visitor,
};
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use strum::Display;

///
/// Arg
///

#[derive(Clone, Debug, Display, Serialize, Deserialize)]
pub enum Arg {
    Bool(bool),
    Char(char),
    Number(ArgNumber),
    Path(String),
    String(String),
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

#[derive(Clone, Debug, Default, Deref, Serialize, Deserialize)]
pub struct Args(pub Vec<Arg>);

impl Args {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl ValidateNode for Args {}

///
/// ArgNumber
///

#[derive(Clone, Debug, Display, Serialize, Deserialize)]
pub enum ArgNumber {
    Float(f64),
    F32(f32),
    F64(f64),
    Integer(i128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
}

impl ValidateNode for ArgNumber {}

impl VisitableNode for ArgNumber {}
