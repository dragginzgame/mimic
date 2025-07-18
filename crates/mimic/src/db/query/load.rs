#![allow(clippy::type_complexity)]
use crate::{
    core::{Value, traits::EntityKind},
    db::query::{Cmp, FilterBuilder, FilterExpr, SortDirection, SortExpr},
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
    #[must_use]
    pub fn one<E: EntityKind>(self, value: impl Into<Value>) -> Self {
        self.filter_eq(E::PRIMARY_KEY, value.into())
    }

    // many
    #[must_use]
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

    #[must_use]
    pub fn filter_eq<F: Into<String>, V: Into<Value>>(self, field: F, value: V) -> Self {
        self.filter(|f| f.eq(field, value))
    }

    #[must_use]
    pub fn filter_eq_opt<F: Into<String>, V: Into<Value>>(
        self,
        field: F,
        value: Option<V>,
    ) -> Self {
        self.filter(|f| f.eq_opt(field, value))
    }

    #[must_use]
    pub fn filter_in<F: Into<String>, V: Into<Value>>(self, field: F, value: V) -> Self {
        self.filter(|f| f.filter(field, Cmp::In, value))
    }

    #[must_use]
    pub fn set_filter(mut self, expr: FilterExpr) -> Self {
        self.filter = Some(expr);
        self
    }

    // filter
    #[must_use]
    pub fn filter(mut self, f: impl FnOnce(FilterBuilder) -> FilterBuilder) -> Self {
        let builder = match self.filter.take() {
            Some(existing) => FilterBuilder::from(existing),
            None => FilterBuilder::new(),
        };

        if let Some(expr) = f(builder).build() {
            self.filter = Some(expr);
        }

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
