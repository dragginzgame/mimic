mod dsl;
mod expr;
mod ext;

pub use dsl::*;
pub use expr::*;
pub use ext::*;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

///
/// Cmp
/// comparator operators for clauses
///

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Cmp {
    // general comparison
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,

    // array matching
    In,
    AllIn,
    AnyIn,

    // text matching
    Contains,
    StartsWith,
    EndsWith,

    // case insensitive matching
    EqCi,
    NeCi,
    ContainsCi,
    StartsWithCi,
    EndsWithCi,

    // optionals / presence
    IsNone,
    IsSome,

    // collections / strings
    IsEmpty,    // len == 0
    IsNotEmpty, // len > 0
}

impl Cmp {
    // compare_order
    // helper function to evaluate an 'Ordering' result against this
    // comparison operator
    #[must_use]
    pub fn compare_order(&self, ord: Ordering) -> bool {
        match self {
            Self::Eq => ord == Ordering::Equal,
            Self::Ne => ord != Ordering::Equal,
            Self::Lt => ord == Ordering::Less,
            Self::Lte => ord != Ordering::Greater,
            Self::Gt => ord == Ordering::Greater,
            Self::Gte => ord != Ordering::Less,
            _ => false,
        }
    }
}
