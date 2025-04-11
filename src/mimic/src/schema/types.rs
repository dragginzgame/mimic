use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Cardinality
///

#[derive(CandidType, Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Cardinality {
    One,
    Opt,
    Many,
}

///
/// ConstantType
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
#[remain::sorted]
pub enum ConstantType {
    Bool,
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Nat128,
    Str,
}

///
/// PrimitiveType
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
#[remain::sorted]
pub enum PrimitiveType {
    Blob,
    Bool,
    Decimal,
    Float32,
    Float64,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Nat,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Nat128,
    Principal,
    SortKey,
    String,
    Ulid,
    Unit,
}

///
/// PrimitiveGroup
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
#[remain::sorted]
pub enum PrimitiveGroup {
    Blob,
    Bool,
    Decimal,
    Float,
    Integer,
    SortKey,
    String,
    Ulid,
    Unit,
}

///
/// SortDirection
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SortDirection {
    Asc,
    Desc,
}
