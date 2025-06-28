#![allow(clippy::type_complexity)]
use crate::{
    core::{types::EntityKey, value::Value},
    db::query::{Cmp, FilterBuilder, FilterClause, FilterExpr, Selector, SortDirection},
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
/// LoadQueryBuilder
///

#[derive(Debug, Default)]
pub struct LoadQueryBuilder {}

impl LoadQueryBuilder {
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // selector
    #[must_use]
    pub fn selector(self, selector: Selector) -> LoadQuery {
        LoadQuery::new(selector)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQuery {
        LoadQuery::new(Selector::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQuery {
        LoadQuery::new(Selector::Only)
    }

    // one
    pub fn one<K: Into<EntityKey>>(self, key: K) -> LoadQuery {
        let selector = Selector::One(key.into());

        LoadQuery::new(selector)
    }

    // many
    #[must_use]
    pub fn many<K>(self, keys: &[K]) -> LoadQuery
    where
        K: Clone + Into<EntityKey>,
    {
        let keys = keys.iter().cloned().map(Into::into).collect();
        let selector = Selector::Many(keys);

        LoadQuery::new(selector)
    }

    // range
    pub fn range<K: Into<EntityKey>>(self, start: K, end: K) -> LoadQuery {
        let selector = Selector::Range(start.into(), end.into());

        LoadQuery::new(selector)
    }

    // prefix
    pub fn prefix<K: Into<EntityKey>>(self, prefix: K) -> LoadQuery {
        let selector = Selector::Prefix(prefix.into());

        LoadQuery::new(selector)
    }
}

///
/// LoadQuery
/// fluent methods are handled in LoadQueryInternal
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoadQuery {
    pub selector: Selector,
    pub format: LoadFormat,
    pub filter: Option<FilterExpr>,
    pub limit: Option<u32>,
    pub offset: u32,
    pub sort: Vec<(String, SortDirection)>,
}

impl LoadQuery {
    #[must_use]
    pub fn new(selector: Selector) -> Self {
        Self {
            selector,
            ..Default::default()
        }
    }

    // format
    #[must_use]
    pub const fn format(mut self, format: LoadFormat) -> Self {
        self.format = format;
        self
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

    // filter_eq
    pub fn filter_eq<F: Into<String>, V: Into<Value>>(mut self, field: F, value: V) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, Cmp::Eq, value));
        self.filter = Some(match self.filter.take() {
            Some(existing) => existing.and(clause),
            None => clause,
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
        self.sort = sort.into_iter().map(|(f, d)| (f.into(), d)).collect();
        self
    }

    // sort_field
    #[must_use]
    pub fn sort_field<K: Into<String>>(self, field: K, dir: SortDirection) -> Self {
        self.sort(std::iter::once((field, dir)))
    }
}
