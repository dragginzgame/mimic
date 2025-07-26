use crate::{
    core::value::Value,
    db::query::{Cmp, FilterClause, FilterExpr},
};

///
/// Logic
///

pub enum Logic {
    And,
    Or,
}

///
/// FilterBuilder
///

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FilterBuilder {
    pub filter: Option<FilterExpr>,
}

///
/// FilterBuilder is a compositional helper for building `Filter` trees incrementally,
/// with automatic flattening of `And`/`Or` logic and support for nested groupings.
///
/// # Purpose
/// This builder abstracts the mechanical construction of `Filter` trees. Instead of manually
/// nesting `Filter::And`, `Or`, `Not`, etc., this provides a fluent interface for progressive
/// filter composition that aligns with common query-building workflows.
///
/// Internally it uses `Filter::{and, or}` methods, which already flatten nested expressions,
/// so this builder preserves logical flattening and avoids unnecessary tree depth.
///
/// # Grouping Semantics
/// Use `filter_group(...)` and `or_filter_group(...)` to build nested subtrees (like `(a AND b)`),
/// which are inserted into the parent with the correct logical context. These are useful for cases
/// like:
/// ```text
///    x = 1 AND (y = 2 OR z = 3)
/// ```
///
/// # Interop with Filter
/// You can inject existing `Filter` trees using `.filter_expr()` or `.or_filter_expr()`,
/// and the builder is also `From<Filter>`-convertible.
///
/// # Output
/// `build()` returns `Option<Filter>`. If no filters were added, it returns `None`.
/// You can simplify the result post-build using `.simplify()` on the resulting `Filter`.
///
/// # Future You Notes
/// - If you’re picking this up later: the core `Filter` enum supports flattening, simplification,
///   and De Morgan-style rewrites. This builder is just a layered ergonomic interface over that system.
/// - No validation or optimization is done here — that lives in `Filter::simplify()`.
/// - See `filter_opt` if you’re adding dynamic/optional filters based on user input.
/// - Tests live alongside and verify grouping, flattening, and common construction patterns.
///

impl FilterBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // add_expr
    fn add_expr(mut self, expr: FilterExpr, logic: Logic) -> Self {
        self.filter = Some(match (self.filter.take(), logic) {
            (Some(existing), Logic::Or) => existing.or(expr),
            (Some(existing), Logic::And) => existing.and(expr),
            (None, _) => expr,
        });
        self
    }

    ///
    /// COMPARISONS
    ///

    #[must_use]
    pub fn eq(self, field: &str, value: impl Into<Value>) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, Cmp::Eq, value));

        self.add_expr(clause, Logic::And)
    }

    #[must_use]
    pub fn eq_opt(self, field: &str, value: Option<impl Into<Value>>) -> Self {
        if let Some(val) = value {
            self.eq(field, val)
        } else {
            self.eq(field, Value::None)
        }
    }

    ///
    /// AND
    ///

    #[must_use]
    pub fn filter(self, field: &str, cmp: Cmp, value: impl Into<Value>) -> Self {
        self.and(field, cmp, value)
    }

    #[must_use]
    pub fn and(self, field: &str, cmp: Cmp, value: impl Into<Value>) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, cmp, value));

        self.add_expr(clause, Logic::And)
    }

    #[must_use]
    pub fn expr(self, expr: FilterExpr) -> Self {
        self.add_expr(expr, Logic::And)
    }

    // alias for expr
    #[must_use]
    pub fn and_expr(self, expr: FilterExpr) -> Self {
        self.expr(expr)
    }

    #[must_use]
    pub fn and_opt(self, field: &str, cmp: Cmp, value: Option<impl Into<Value>>) -> Self {
        match value {
            Some(v) => self.and(field, cmp, v),
            None => self,
        }
    }

    #[must_use]
    pub fn and_group<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        match f(Self::new()).build() {
            Some(expr) => self.add_expr(expr, Logic::And),
            None => self,
        }
    }

    ///
    /// OR
    ///

    #[must_use]
    pub fn or(self, field: &str, cmp: Cmp, value: impl Into<Value>) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, cmp, value));

        self.add_expr(clause, Logic::Or)
    }

    #[must_use]
    pub fn or_expr(self, expr: FilterExpr) -> Self {
        self.add_expr(expr, Logic::Or)
    }

    #[must_use]
    pub fn or_opt(self, field: &str, cmp: Cmp, value: Option<impl Into<Value>>) -> Self {
        match value {
            Some(v) => self.or(field, cmp, v),
            None => self,
        }
    }

    #[must_use]
    pub fn or_group<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        match f(Self::new()).build() {
            Some(expr) => self.add_expr(expr, Logic::Or),
            None => self,
        }
    }

    ///
    /// NOT
    ///

    #[must_use]
    pub fn not(self, field: &str, cmp: Cmp, value: impl Into<Value>) -> Self {
        let clause = FilterExpr::Clause(FilterClause::new(field, cmp, value));
        self.not_expr(clause)
    }

    #[must_use]
    pub fn not_expr(self, expr: FilterExpr) -> Self {
        self.and_expr(FilterExpr::Not(Box::new(expr)))
    }

    #[must_use]
    pub fn not_group(self, f: impl FnOnce(Self) -> Self) -> Self {
        match f(Self::new()).build() {
            Some(expr) => self.and_expr(FilterExpr::Not(Box::new(expr))),
            None => self,
        }
    }

    #[must_use]
    pub fn not_opt(self, field: &str, cmp: Cmp, value: Option<impl Into<Value>>) -> Self {
        match value {
            Some(v) => self.not(field, cmp, v),
            None => self,
        }
    }

    ///
    /// OTHER
    ///

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.filter.is_none()
    }

    #[must_use]
    pub fn build(self) -> Option<FilterExpr> {
        self.filter
    }

    #[must_use]
    pub fn build_and_simplify(self) -> Option<FilterExpr> {
        self.build().map(FilterExpr::simplify)
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
        let filter = FilterBuilder::new().and("a", Cmp::Eq, 42).build().unwrap();

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
            .and("a", Cmp::Eq, 1)
            .and("b", Cmp::Eq, 2)
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
            .and("a", Cmp::Eq, 1)
            .or("b", Cmp::Eq, 2)
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
            .and("top", Cmp::Eq, true)
            .or_group(|b| b.and("x", Cmp::Eq, "A").and("y", Cmp::Eq, "B"))
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
        let builder = FilterBuilder::new().and("x", Cmp::Eq, 1).clear();

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
            .and("a", Cmp::Eq, 1)
            .and_expr(FilterExpr::True)
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
            .and_group(|b| {
                b.and("a", Cmp::Eq, 1)
                    .and_group(|b| b.and("b", Cmp::Eq, 2).and("c", Cmp::Eq, 3))
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

    #[test]
    fn not_clause_works() {
        let filter = FilterBuilder::new()
            .not("active", Cmp::Eq, false)
            .build()
            .unwrap();

        match filter {
            FilterExpr::Not(inner) => match *inner {
                FilterExpr::Clause(c) => {
                    assert_eq!(c.field, "active");
                    assert_eq!(c.cmp, Cmp::Eq);
                    assert_eq!(c.value, Value::from(false));
                }
                _ => panic!("Expected Clause inside Not"),
            },
            _ => panic!("Expected Not expression"),
        }
    }

    #[test]
    fn not_group_combines_multiple_clauses() {
        let filter = FilterBuilder::new()
            .not_group(|b| b.and("x", Cmp::Eq, 1).and("y", Cmp::Eq, 2))
            .build()
            .unwrap();

        match filter {
            FilterExpr::Not(inner) => match *inner {
                FilterExpr::And(children) => {
                    assert_eq!(children.len(), 2);
                }
                _ => panic!("Expected And inside Not"),
            },
            _ => panic!("Expected Not at root"),
        }
    }

    #[test]
    fn and_opt_includes_only_some() {
        let filter = FilterBuilder::new()
            .and_opt("a", Cmp::Eq, Some(1))
            .and_opt("b", Cmp::Eq, Option::<i32>::None)
            .build()
            .unwrap();

        match filter {
            FilterExpr::Clause(c) => {
                assert_eq!(c.field, "a");
            }
            _ => panic!("Expected single Clause"),
        }
    }

    #[test]
    fn or_opt_combines_optional() {
        let filter = FilterBuilder::new()
            .or("x", Cmp::Eq, 1)
            .or_opt("y", Cmp::Eq, Some(2))
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
    fn not_opt_skips_none() {
        let filter = FilterBuilder::new()
            .not_opt("a", Cmp::Eq, Option::<i32>::None)
            .build();

        assert_eq!(filter, None);
    }

    #[test]
    fn build_and_simplify_removes_true() {
        let filter = FilterBuilder::new()
            .and_expr(FilterExpr::True)
            .and("a", Cmp::Eq, 1)
            .build_and_simplify()
            .unwrap();

        match filter {
            FilterExpr::Clause(c) => {
                assert_eq!(c.field, "a");
            }
            _ => panic!("Expected Clause after simplification"),
        }
    }

    #[test]
    fn is_empty_true_on_new() {
        let builder = FilterBuilder::new();
        assert!(builder.is_empty());
    }

    #[test]
    fn is_empty_false_when_clause_added() {
        let builder = FilterBuilder::new().filter("a", Cmp::Eq, 1);
        assert!(!builder.is_empty());
    }
}
