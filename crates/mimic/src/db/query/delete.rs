use crate::{
    core::{Value, traits::EntityKind},
    db::query::{Cmp, FilterClause, FilterExpr, RangeExpr},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// DeleteQuery
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeleteQuery {
    pub range: Option<RangeExpr>,
    pub filter: Option<FilterExpr>,
}

impl DeleteQuery {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn one<E: EntityKind>(self, value: impl Into<Value>) -> Self {
        self.filter_eq(E::PRIMARY_KEY, value.into())
    }

    pub fn many<E, V>(self, values: &[V]) -> Self
    where
        E: EntityKind,
        V: Clone + Into<Value>,
    {
        let list = values
            .iter()
            .cloned()
            .map(|v| Box::new(v.into()))
            .collect::<Vec<_>>();

        self.filter_in(E::PRIMARY_KEY, Value::List(list))
    }

    // filter_in
    pub fn filter_in<F: Into<String>, V: Into<Value>>(self, field: F, value: V) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, Cmp::In, value));

        self.merge_filter(clause)
    }

    // filter_eq
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
