use serde::{Deserialize, Serialize};

///
/// Cardinality
///

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Cardinality {
    One,
    Opt,
    Many,
}

///
/// CrudAction
///

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CrudAction {
    Load,
    Save,
    Delete,
}

///
/// Cycles
///

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Cycles(pub u128);

///
/// PrimitiveType
///

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[remain::sorted]
pub enum PrimitiveType {
    Blob,
    Bool,
    Decimal,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    Principal,
    String,
    Timestamp,
    U8,
    U16,
    U32,
    U64,
    U128,
    Ulid,
    Usize,
}

///
/// PrimitiveGroup
///

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[remain::sorted]
pub enum PrimitiveGroup {
    Blob,
    Bool,
    Decimal,
    Float,
    Integer,
    String,
}

///
/// SortDirection
///

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SortDirection {
    Asc,
    Desc,
}
