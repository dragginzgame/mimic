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

    // combine
    fn combine(mut self, expr: FilterExpr, use_or: bool) -> Self {
        self.filter = Some(match self.filter.take() {
            Some(existing) if use_or => existing.or(expr),
            Some(existing) => existing.and(expr),
            None => expr,
        });

        self
    }

    #[must_use]
    pub fn filter<F: Into<String>, V: Into<Value>>(self, field: F, cmp: Cmp, value: V) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, cmp, value));

        self.combine(clause, false)
    }

    #[must_use]
    pub fn or_filter<F: Into<String>, V: Into<Value>>(self, field: F, cmp: Cmp, value: V) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, cmp, value));

        self.combine(clause, true)
    }

    #[must_use]
    pub fn filter_expr(self, expr: FilterExpr) -> Self {
        self.combine(expr, false)
    }

    #[must_use]
    pub fn or_filter_expr(self, expr: FilterExpr) -> Self {
        self.combine(expr, true)
    }

    #[must_use]
    pub fn filter_group<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        match f(Self::new()).build() {
            Some(expr) => self.combine(expr, false),
            None => self,
        }
    }

    #[must_use]
    pub fn or_filter_group<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        match f(Self::new()).build() {
            Some(expr) => self.combine(expr, true),
            None => self,
        }
    }

    #[must_use]
    pub fn build(self) -> Option<FilterExpr> {
        self.filter
    }

    #[must_use]
    pub fn clear(mut self) -> Self {
        self.filter = None;

        self
    }
}

impl From<FilterExpr> for FilterBuilder {
    fn from(expr: FilterExpr) -> Self {
        Self { filter: Some(expr) }
    }
}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;

    fn clause(field: &str, value: impl Into<Value>) -> FilterExpr {
        FilterExpr::Clause(FilterClause::new(field, Cmp::Eq, value))
    }

    #[test]
    fn builds_single_clause() {
        let filter = FilterBuilder::new()
            .filter("a", Cmp::Eq, 42)
            .build()
            .unwrap();

        match filter {
            FilterExpr::Clause(c) => {
                assert_eq!(c.field, "a");
                assert_eq!(c.cmp, Cmp::Eq);
                assert_eq!(c.value, Value::from(42));
            }
            _ => panic!("Expected Clause"),
        }
    }

    #[test]
    fn chains_and_clauses() {
        let filter = FilterBuilder::new()
            .filter("a", Cmp::Eq, 1)
            .filter("b", Cmp::Eq, 2)
            .build()
            .unwrap();

        match filter {
            FilterExpr::And(children) => {
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected And"),
        }
    }

    #[test]
    fn chains_or_clauses() {
        let filter = FilterBuilder::new()
            .filter("a", Cmp::Eq, 1)
            .or_filter("b", Cmp::Eq, 2)
            .build()
            .unwrap();

        match filter {
            FilterExpr::Or(children) => {
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected Or"),
        }
    }

    #[test]
    fn groups_and_or_mixed() {
        let filter = FilterBuilder::new()
            .filter("top", Cmp::Eq, true)
            .or_filter_group(|b| b.filter("x", Cmp::Eq, "A").filter("y", Cmp::Eq, "B"))
            .build()
            .unwrap();

        match filter {
            FilterExpr::Or(children) => {
                assert_eq!(children.len(), 2);
                if let FilterExpr::And(grouped) = &children[1] {
                    assert_eq!(grouped.len(), 2);
                } else {
                    panic!("Expected grouped And inside Or");
                }
            }
            _ => panic!("Expected Or at root"),
        }
    }

    #[test]
    fn from_expr_works() {
        let expr = clause("a", 123);
        let builder = FilterBuilder::from(expr.clone());
        assert_eq!(builder.build(), Some(expr));
    }

    #[test]
    fn clear_resets_filter() {
        let builder = FilterBuilder::new().filter("x", Cmp::Eq, 1).clear();

        assert_eq!(builder.build(), None);
    }

    #[test]
    fn build_empty_returns_none() {
        let builder = FilterBuilder::new();
        assert_eq!(builder.build(), None);
    }

    #[test]
    fn simplify_built_expr() {
        let expr = FilterBuilder::new()
            .filter("a", Cmp::Eq, 1)
            .filter_expr(FilterExpr::True)
            .build()
            .unwrap()
            .simplify();

        match expr {
            FilterExpr::Clause(c) => assert_eq!(c.field, "a"),
            _ => panic!("Expected simplified Clause"),
        }
    }

    #[test]
    fn nested_groups_flatten() {
        let filter = FilterBuilder::new()
            .filter_group(|b| {
                b.filter("a", Cmp::Eq, 1)
                    .filter_group(|b| b.filter("b", Cmp::Eq, 2).filter("c", Cmp::Eq, 3))
            })
            .build()
            .unwrap()
            .simplify();

        match filter {
            FilterExpr::And(children) => {
                assert_eq!(children.len(), 3);
            }
            _ => panic!("Expected flattened And"),
        }
    }
}
