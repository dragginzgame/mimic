#![allow(clippy::type_complexity)]
use crate::{
    core::{Key, Value, traits::EntityKind},
    db::query::{Cmp, FilterBuilder, FilterClause, FilterExpr, RangeExpr, SortDirection, SortExpr},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadFormat
///

#[derive(CandidType, Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum LoadFormat {
    #[default]
    Keys,
    Count,
}

///
/// LoadQuery
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoadQuery {
    pub format: LoadFormat,
    pub filter: Option<FilterExpr>,
    pub range: Option<RangeExpr>,
    pub limit: Option<u32>,
    pub offset: u32,
    pub sort: Option<SortExpr>,
}

impl LoadQuery {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // format
    #[must_use]
    pub const fn format(mut self, format: LoadFormat) -> Self {
        self.format = format;
        self
    }

    // one
    pub fn one<E: EntityKind>(self, key: impl Into<Key>) -> Self {
        self.filter_eq(E::PRIMARY_KEY, key.into())
    }

    // many
    pub fn many<E: EntityKind, K>(self, keys: &[K]) -> Self
    where
        K: Into<Key> + Clone,
    {
        let list = keys
            .iter()
            .cloned()
            .map(|k| Box::new(Value::from(k.into())))
            .collect::<Vec<_>>();

        let value = Value::List(list);
        let clause = FilterExpr::Clause(FilterClause::new(E::PRIMARY_KEY, Cmp::In, value));

        self.merge_filter(clause)
    }

    // with_filter
    // use an external builder to replace the current Filter
    #[must_use]
    pub fn with_filter(mut self, f: impl FnOnce(FilterBuilder) -> FilterBuilder) -> Self {
        if let Some(expr) = f(FilterBuilder::new()).build() {
            self.filter = Some(expr);
        }

        self
    }

    #[must_use]
    pub fn filter_eq<F: Into<String>, V: Into<Value>>(self, field: F, value: V) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, Cmp::Eq, value));

        self.merge_filter(clause)
    }

    #[must_use]
    pub fn filter_in<F: Into<String>, V: Into<Value>>(self, field: F, value: V) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, Cmp::In, value));

        self.merge_filter(clause)
    }

    #[must_use]
    pub fn set_filter(mut self, expr: FilterExpr) -> Self {
        self.filter = Some(expr);
        self
    }

    fn merge_filter(mut self, new: FilterExpr) -> Self {
        self.filter = Some(match self.filter.take() {
            Some(existing) => existing.and(new),
            None => new,
        });

        self
    }

    // offset
    #[must_use]
    pub const fn offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    // limit
    #[must_use]
    pub const fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    // limit_option
    #[must_use]
    pub const fn limit_option(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;
        self
    }

    // sort
    #[must_use]
    pub fn sort<T, I>(mut self, sort: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = (T, SortDirection)>,
    {
        if let Some(expr) = &mut self.sort {
            expr.extend(sort);
        } else {
            self.sort = Some(SortExpr::from_iter(sort));
        }

        self
    }

    // sort_field
    #[must_use]
    pub fn sort_field<K: Into<String>>(self, field: K, dir: SortDirection) -> Self {
        self.sort(std::iter::once((field, dir)))
    }
}
