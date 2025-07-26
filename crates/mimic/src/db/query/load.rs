use crate::{
    core::{Value, traits::EntityKind},
    db::query::{Cmp, FilterBuilder, FilterExpr, SortDirection, SortExpr},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadQuery
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoadQuery {
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

    // one
    #[must_use]
    pub fn one<E: EntityKind>(self, value: impl Into<Value>) -> Self {
        self.filter_eq(E::PRIMARY_KEY, value.into())
    }

    // many
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

    #[must_use]
    pub fn filter_eq(self, field: &str, value: impl Into<Value>) -> Self {
        self.filter(|f| f.eq(field, value))
    }

    #[must_use]
    pub fn filter_eq_opt(self, field: &str, value: Option<impl Into<Value>>) -> Self {
        self.filter(|f| f.eq_opt(field, value))
    }

    #[must_use]
    pub fn filter_in(self, field: &str, value: impl Into<Value>) -> Self {
        self.filter(|f| f.filter(field, Cmp::In, value))
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
    pub fn sort_field(self, field: &str, dir: SortDirection) -> Self {
        self.sort(std::iter::once((field, dir)))
    }
}
