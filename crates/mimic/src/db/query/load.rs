use crate::{
    core::{Value, traits::EntityKind},
    db::query::{
        FilterExpr, FilterSlot, LimitExpr, LimitSlot, QueryError, QueryValidate, SortExpr,
        SortSlot, prelude::*,
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadQuery
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoadQuery {
    pub filter: Option<FilterExpr>,
    pub limit: Option<LimitExpr>,
    pub sort: Option<SortExpr>,
}

impl LoadQuery {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.filter.is_none() && self.limit.is_none() && self.sort.is_none()
    }

    ///
    /// SHAPES
    ///

    #[must_use]
    pub fn one<E: EntityKind>(self, value: impl Into<Value>) -> Self {
        self.filter(|f| f.eq(E::PRIMARY_KEY, value))
    }

    #[must_use]
    pub fn many<E: EntityKind>(self, values: impl IntoIterator<Item = impl Into<Value>>) -> Self {
        self.filter(move |f| f.in_iter(E::PRIMARY_KEY, values))
    }

    // all just overrides, same as calling new
    #[must_use]
    pub fn all() -> Self {
        Self::default()
    }
}

impl FilterSlot for LoadQuery {
    fn filter_slot(&mut self) -> &mut Option<FilterExpr> {
        &mut self.filter
    }
}

impl LimitSlot for LoadQuery {
    fn limit_slot(&mut self) -> &mut Option<LimitExpr> {
        &mut self.limit
    }
}

impl SortSlot for LoadQuery {
    fn sort_slot(&mut self) -> &mut Option<SortExpr> {
        &mut self.sort
    }
}

impl<E: EntityKind> QueryValidate<E> for LoadQuery {
    fn validate(&self) -> Result<(), QueryError> {
        if let Some(filter) = &self.filter {
            QueryValidate::<E>::validate(filter)?;
        }
        if let Some(limit) = &self.limit {
            QueryValidate::<E>::validate(limit)?;
        }
        if let Some(sort) = &self.sort {
            QueryValidate::<E>::validate(sort)?;
        }

        Ok(())
    }
}
