use crate::{
    db::executor::ResolvedSelector,
    ops::{Value, traits::EntityKind},
    types::EntityKey,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Selector
///
/// All    : no sort key prefix, only works with top-level Sort Keys
/// Only   : for entities that have no keys
/// One    : returns one row by composite key
/// Many   : returns many rows (from many keys)
/// Prefix : like all but we're asking for the key prefix
///          so Pet (Character=1) will return the Pets from Character 1
/// Range  : user-defined range, ie. Item=1000 Item=1500
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub enum Selector {
    #[default]
    All,
    Only,
    One(EntityKey),
    Many(Vec<EntityKey>),
    Prefix(EntityKey),
    Range(EntityKey, EntityKey),
}

impl Selector {
    #[must_use]
    pub fn resolve<E: EntityKind>(&self) -> ResolvedSelector {
        match self {
            Self::All => {
                let start = E::build_data_key(&[]);
                let end = start.create_upper_bound();

                ResolvedSelector::Range(start, end)
            }
            Self::Only => ResolvedSelector::One(E::build_data_key(&[])),
            Self::One(key) => ResolvedSelector::One(E::build_data_key(key)),
            Self::Many(keys) => {
                let keys = keys.iter().map(|k| E::build_data_key(k)).collect();

                ResolvedSelector::Many(keys)
            }
            Self::Prefix(prefix) => {
                let start = E::build_data_key(prefix);
                let end = start.create_upper_bound();

                ResolvedSelector::Range(start, end)
            }
            Self::Range(start, end) => {
                let start = E::build_data_key(start);
                let end = E::build_data_key(end);

                ResolvedSelector::Range(start, end)
            }
        }
    }
}

///
/// SortDirection
///

#[derive(CandidType, Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

///
/// WhereExpr
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub enum WhereExpr {
    Clause(WhereClause),
    And(Vec<WhereExpr>),
    Or(Vec<WhereExpr>),
    Not(Box<WhereExpr>),
}

impl WhereExpr {
    /// Combines this expression with another using `And`.
    #[must_use]
    pub fn and(self, other: WhereExpr) -> Self {
        match self {
            WhereExpr::And(mut children) => {
                children.push(other);
                WhereExpr::And(children)
            }
            _ => WhereExpr::And(vec![self, other]),
        }
    }
}

///
/// WhereClause
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct WhereClause {
    pub field: String,
    pub cmp: Comparator,
    pub value: Value,
}

impl WhereClause {
    pub fn new<F: Into<String>, V: Into<Value>>(field: F, cmp: Comparator, value: V) -> Self {
        Self {
            field: field.into(),
            cmp,
            value: value.into(),
        }
    }
}

///
/// Comparator
///

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Comparator {
    Eq,
    Ne,
    Lt,
    Ltoe,
    Gt,
    Gtoe,
}
