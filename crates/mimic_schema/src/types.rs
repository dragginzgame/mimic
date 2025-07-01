use candid::CandidType;
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};

///
/// Cardinality
///

#[derive(
    CandidType, Clone, Copy, Default, Debug, Deserialize, Display, Eq, FromStr, PartialEq, Serialize,
)]
pub enum Cardinality {
    #[default]
    One,
    Opt,
    Many,
}

///
/// ConstantType
///

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Display, FromStr, Serialize)]
#[remain::sorted]
pub enum ConstantType {
    Bool,
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Str,
}

///
/// Primitive
///

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Display, FromStr, Serialize)]
#[remain::sorted]
pub enum Primitive {
    Account,
    Blob,
    Bool,
    Decimal,
    E8s,
    E18s,
    Float32,
    Float64,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Nat,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Principal,
    Subaccount,
    Text,
    Ulid,
    Unit,
}

///
/// StoreType
///

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Display, FromStr, Serialize)]
pub enum StoreType {
    Data,
    Index,
}
