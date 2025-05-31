use crate::schema::{
    node::{ValidateNode, VisitableNode},
    visit::Visitor,
};
use derive_more::{Deref, Display};
use serde::{Deserialize, Serialize};

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
    pub const fn is_empty(&self) -> bool {
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
    Float32(f32),
    Float64(f64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Nat8(u8),
    Nat16(u16),
    Nat32(u32),
    Nat64(u64),
    Nat128(u128),
}

impl ValidateNode for ArgNumber {}

impl VisitableNode for ArgNumber {}
