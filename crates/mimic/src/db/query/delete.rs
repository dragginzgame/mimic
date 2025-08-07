use crate::{
    core::{Value, traits::EntityKind},
    db::query::{Cmp, FilterBuilder, FilterExpr, LimitExpr, QueryError, QueryValidate},
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
    pub fn one<E: EntityKind>(self, value: impl Into<Value>) -> Self {
        self.filter_eq(E::PRIMARY_KEY, value.into())
    }

    #[must_use]
    pub fn many<E: EntityKind>(self, values: impl IntoIterator<Item = impl Into<Value>>) -> Self {
        let list = values
            .into_iter()
            .map(|v| Box::new(v.into()))
            .collect::<Vec<_>>();

        self.filter_in(E::PRIMARY_KEY, Value::List(list))
    }

    ///
    /// FILTER
    ///

    /// Combines the existing filter expression with a new one using `And`.
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
    pub fn offset(mut self, offset: u32) -> Self {
        let expr = self.limit.unwrap_or_default().offset(offset);
        self.limit = Some(expr);
        self
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
