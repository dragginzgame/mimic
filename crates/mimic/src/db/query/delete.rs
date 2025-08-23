use crate::{
    core::traits::{EntityKind, FieldValue},
    db::query::{
        FilterExpr, FilterExt, FilterSlot, LimitExpr, LimitSlot, QueryError, QueryValidate,
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// DeleteQuery
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeleteQuery {
    pub filter: Option<FilterExpr>,
    pub limit: Option<LimitExpr>,
}

impl DeleteQuery {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn one<E: EntityKind>(self, value: impl FieldValue) -> Self {
        self.filter(|f| f.eq(E::PRIMARY_KEY, value))
    }

    #[must_use]
    pub fn many<E: EntityKind>(self, values: impl IntoIterator<Item = impl FieldValue>) -> Self {
        self.filter(move |f| f.in_iter(E::PRIMARY_KEY, values))
    }
}

impl FilterSlot for DeleteQuery {
    fn filter_slot(&mut self) -> &mut Option<FilterExpr> {
        &mut self.filter
    }
}

impl LimitSlot for DeleteQuery {
    fn limit_slot(&mut self) -> &mut Option<LimitExpr> {
        &mut self.limit
    }
}

impl<E: EntityKind> QueryValidate<E> for DeleteQuery {
    fn validate(&self) -> Result<(), QueryError> {
        if let Some(filter) = &self.filter {
            QueryValidate::<E>::validate(filter)?;
        }

        if let Some(limit) = &self.limit {
            QueryValidate::<E>::validate(limit)?;
        }

        Ok(())
    }
}
