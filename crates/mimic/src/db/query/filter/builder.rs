use crate::{
    core::value::Value,
    db::query::{Cmp, FilterClause, FilterExpr},
};

///
/// FilterBuilder
///

#[derive(Clone, Debug, Default)]
pub struct FilterBuilder {
    pub filter: Option<FilterExpr>,
}

impl FilterBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn filter<F: Into<String>, V: Into<Value>>(mut self, field: F, cmp: Cmp, value: V) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, cmp, value));
        self.filter = Some(match self.filter {
            Some(existing) => existing.and(clause),
            None => clause,
        });
        self
    }

    #[must_use]
    pub fn or_filter<F: Into<String>, V: Into<Value>>(
        mut self,
        field: F,
        cmp: Cmp,
        value: V,
    ) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, cmp, value));
        self.filter = Some(match self.filter {
            Some(existing) => existing.or(clause),
            None => clause,
        });
        self
    }

    #[must_use]
    pub fn filter_expr(mut self, expr: FilterExpr) -> Self {
        self.filter = Some(match self.filter {
            Some(existing) => existing.and(expr),
            None => expr,
        });
        self
    }

    #[must_use]
    pub fn or_filter_expr(mut self, expr: FilterExpr) -> Self {
        self.filter = Some(match self.filter {
            Some(existing) => existing.or(expr),
            None => expr,
        });
        self
    }

    #[must_use]
    pub fn filter_group<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        match f(Self::new()).filter {
            Some(expr) => self.filter_expr(expr),
            None => self,
        }
    }

    #[must_use]
    pub fn or_filter_group<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        match f(Self::new()).filter {
            Some(expr) => self.or_filter_expr(expr),
            None => self,
        }
    }

    #[must_use]
    pub fn build(self) -> Option<FilterExpr> {
        self.filter
    }
}
