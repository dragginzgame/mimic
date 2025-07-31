use crate::{
    core::{Value, traits::EntityKind},
    db::query::{
        Cmp, FilterBuilder, FilterExpr, LimitExpr, QueryError, QueryValidate, SortDirection,
        SortExpr,
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
    pub fn is_empty(&self) -> bool {
        self.filter.is_none() && self.limit.is_none() && self.sort.is_none()
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

    ///
    /// LIMIT
    ///

    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        let expr = self.limit.unwrap_or_default().limit(limit);
        self.limit = Some(expr);
        self
    }

    #[must_use]
    pub fn limit_option(mut self, limit: Option<u32>) -> Self {
        let mut expr = self.limit.unwrap_or_default();
        expr.limit = limit;
        self.limit = Some(expr);
        self
    }

    #[must_use]
    pub fn offset(mut self, offset: u32) -> Self {
        let expr = self.limit.unwrap_or_default().offset(offset);
        self.limit = Some(expr);
        self
    }

    ///
    /// SORT
    ///

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
