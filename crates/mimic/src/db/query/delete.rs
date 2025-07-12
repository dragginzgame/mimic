use crate::{
    core::{Key, Value},
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
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // one
    pub fn one<K: Into<Key>>(self, key: K) -> Self {
        self.filter_eq("id", key.into())
    }

    // filter_eq
    #[must_use]
    pub fn filter_eq<F: Into<String>, V: Into<Value>>(mut self, field: F, value: V) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, Cmp::Eq, value));
        self.filter = Some(match self.filter.take() {
            Some(existing) => existing.and(clause),
            None => clause,
        });

        self
    }

    // many
    /*
        pub fn many<K>(mut self, keys: &[K]) -> Self
        where
            K: Into<Value> + Clone,
        {
            let values: Vec<Value> = keys.iter().cloned().map(Into::into).collect();
            let clause = Filter::Clause(FilterClause::new("id", Cmp::Contains, values));

            self.filter = Some(clause.and_option(self.filter.take()));

            self
        }
    */
}
