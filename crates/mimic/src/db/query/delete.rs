use crate::{
    core::{Value, traits::EntityKind},
    db::query::{Cmp, FilterClause, FilterExpr},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// DeleteQuery
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeleteQuery {
    pub filter: Option<FilterExpr>,
}

impl DeleteQuery {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn one<E: EntityKind>(self, value: impl Into<Value>) -> Self {
        self.filter_eq(E::PRIMARY_KEY, value.into())
    }

    #[must_use]
    pub fn many<E, I>(self, values: I) -> Self
    where
        E: EntityKind,
        I: IntoIterator,
        I::Item: Into<Value>,
    {
        let list = values
            .into_iter()
            .map(|v| Box::new(v.into()))
            .collect::<Vec<_>>();

        self.filter_in(E::PRIMARY_KEY, Value::List(list))
    }

    // filter_in
    #[must_use]
    pub fn filter_in<F: Into<String>, V: Into<Value>>(self, field: F, value: V) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, Cmp::In, value));

        self.merge_filter(clause)
    }

    // filter_eq
    #[must_use]
    pub fn filter_eq<F: Into<String>, V: Into<Value>>(self, field: F, value: V) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, Cmp::Eq, value));

        self.merge_filter(clause)
    }

    fn merge_filter(mut self, new: FilterExpr) -> Self {
        self.filter = Some(match self.filter.take() {
            Some(existing) => existing.and(new),
            None => new,
        });

        self
    }
}
