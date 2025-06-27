use crate::ops::Value;
use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Represents logical expressions for querying/filtering data.
///
/// Expressions can be:
/// - `True` or `False` constants
/// - Single clauses comparing a field with a value
/// - Composite expressions: `And`, `Or`, and negation `Not`.
#[derive(CandidType, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum WhereExpr {
    True,
    False,
    Clause(WhereClause),
    And(Vec<WhereExpr>),
    Or(Vec<WhereExpr>),
    Not(Box<WhereExpr>),
}

impl WhereExpr {
    /// Combine two expressions into an `And` expression.
    ///
    /// This flattens nested `And`s to avoid deep nesting (e.g., `(a AND b) AND c` becomes `AND[a,b,c]`).
    #[must_use]
    pub fn and(self, other: WhereExpr) -> Self {
        match (self, other) {
            (WhereExpr::And(mut a), WhereExpr::And(mut b)) => {
                a.append(&mut b);
                WhereExpr::And(a)
            }
            (WhereExpr::And(mut a), b) => {
                a.push(b);
                WhereExpr::And(a)
            }
            (a, WhereExpr::And(mut b)) => {
                let mut list = vec![a];
                list.append(&mut b);
                WhereExpr::And(list)
            }
            (a, b) => WhereExpr::And(vec![a, b]),
        }
    }

    /// Negate this expression.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        WhereExpr::Not(Box::new(self))
    }

    /// Combine two expressions into an `Or` expression,
    /// flattening nested `Or`s similarly to `and`.
    #[must_use]
    pub fn or(self, other: WhereExpr) -> Self {
        match (self, other) {
            (WhereExpr::Or(mut a), WhereExpr::Or(mut b)) => {
                a.append(&mut b);
                WhereExpr::Or(a)
            }
            (WhereExpr::Or(mut a), b) => {
                a.push(b);
                WhereExpr::Or(a)
            }
            (a, WhereExpr::Or(mut b)) => {
                let mut list = vec![a];
                list.append(&mut b);
                WhereExpr::Or(list)
            }
            (a, b) => WhereExpr::Or(vec![a, b]),
        }
    }

    /// Simplifies the logical expression recursively, applying rules like:
    /// - Eliminate double negation `NOT NOT x` -> `x`
    /// - Apply De Morgan's laws:
    ///   - `NOT (AND [a, b])` -> `OR [NOT a, NOT b]`
    ///   - `NOT (OR [a, b])` -> `AND [NOT a, NOT b]`
    /// - Flatten nested `And` and `Or` expressions
    /// - Remove neutral elements:
    ///   - `AND [True, x]` -> `x`
    ///   - `OR [False, x]` -> `x`
    /// - Short circuit on constants:
    ///   - `AND` with `False` -> `False`
    ///   - `OR` with `True` -> `True`
    #[must_use]
    pub fn simplify(self) -> Self {
        match self {
            WhereExpr::Not(inner) => match *inner {
                WhereExpr::True => WhereExpr::False,
                WhereExpr::False => WhereExpr::True,
                WhereExpr::Not(inner2) => *inner2.simplify_boxed(), // Double negation elimination
                WhereExpr::And(children) => {
                    // De Morgan's: NOT(AND(...)) == OR(NOT(...))
                    WhereExpr::Or(children.into_iter().map(|c| c.not().simplify()).collect())
                }
                WhereExpr::Or(children) => {
                    // De Morgan's: NOT(OR(...)) == AND(NOT(...))
                    WhereExpr::And(children.into_iter().map(|c| c.not().simplify()).collect())
                }
                x => WhereExpr::Not(Box::new(x.simplify())),
            },

            WhereExpr::And(children) => {
                // Recursively simplify and flatten `And` children
                let flat = Self::simplify_children(children, |e| matches!(e, WhereExpr::And(_)));

                // If any child is `False`, whole AND is False (short circuit)
                if flat.iter().any(|e| matches!(e, WhereExpr::False)) {
                    WhereExpr::False
                } else {
                    // Remove neutral elements `True`
                    let filtered: Vec<_> = flat
                        .into_iter()
                        .filter(|e| !matches!(e, WhereExpr::True))
                        .collect();

                    // If empty after filtering, all were True -> return True
                    match filtered.len() {
                        0 => WhereExpr::True,
                        1 => filtered.into_iter().next().unwrap(),
                        _ => WhereExpr::And(filtered),
                    }
                }
            }

            WhereExpr::Or(children) => {
                // Recursively simplify and flatten `Or` children
                let flat = Self::simplify_children(children, |e| matches!(e, WhereExpr::Or(_)));

                // If any child is `True`, whole OR is True (short circuit)
                if flat.iter().any(|e| matches!(e, WhereExpr::True)) {
                    WhereExpr::True
                } else {
                    // Remove neutral elements `False`
                    let filtered: Vec<_> = flat
                        .into_iter()
                        .filter(|e| !matches!(e, WhereExpr::False))
                        .collect();

                    // If empty after filtering, all were False -> return False
                    match filtered.len() {
                        0 => WhereExpr::False,
                        1 => filtered.into_iter().next().unwrap(),
                        _ => WhereExpr::Or(filtered),
                    }
                }
            }

            // Clauses and constants are already simplest forms
            x => x,
        }
    }

    /// Helper to simplify and flatten nested `And` or `Or` children.
    ///
    /// - `children`: the children expressions to simplify and flatten
    /// - `flatten_if`: a predicate to decide if the child should be flattened
    fn simplify_children(
        children: Vec<WhereExpr>,
        flatten_if: fn(&WhereExpr) -> bool,
    ) -> Vec<WhereExpr> {
        let mut flat = Vec::with_capacity(children.len());

        for child in children {
            let simplified = child.simplify();
            if flatten_if(&simplified) {
                if let WhereExpr::And(nested) | WhereExpr::Or(nested) = simplified {
                    flat.extend(nested);
                }
            } else {
                flat.push(simplified);
            }
        }

        flat
    }

    /// Simplify and return boxed expression (helper for double negation case)
    fn simplify_boxed(self) -> Box<WhereExpr> {
        Box::new(self.simplify())
    }
}

///
/// WhereClause represents a basic comparison expression: `field cmp value`
///
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WhereClause {
    pub field: String,
    pub cmp: Comparator,
    pub value: Value,
}

impl WhereClause {
    pub fn new<F: Into<String>, V: Into<Value>>(field: F, cmp: Comparator, value: V) -> Self {
        Self {
            field: field.into(),
            cmp,
            value: value.into(),
        }
    }
}

///
/// Comparator operators for clauses.
///
#[derive(CandidType, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Comparator {
    Eq,
    Ne,
    Lt,
    Ltoe,
    Gt,
    Gtoe,
}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;

    fn clause(field: &str) -> WhereExpr {
        WhereExpr::Clause(WhereClause::new(field, Comparator::Eq, "foo"))
    }

    #[test]
    fn test_simplify_and_true() {
        let expr = WhereExpr::And(vec![WhereExpr::True, clause("a")]);
        assert!(matches!(expr.simplify(), WhereExpr::Clause(_)));
    }

    #[test]
    fn test_simplify_and_false() {
        let expr = WhereExpr::And(vec![clause("a"), WhereExpr::False]);
        assert_eq!(expr.simplify(), WhereExpr::False);
    }

    #[test]
    fn test_double_negation() {
        let expr = WhereExpr::Not(Box::new(WhereExpr::Not(Box::new(clause("x")))));
        let simplified = expr.simplify();
        assert!(matches!(simplified, WhereExpr::Clause(_)));
    }

    #[test]
    fn test_nested_and_or_flatten() {
        let expr = WhereExpr::And(vec![
            clause("a"),
            WhereExpr::And(vec![clause("b"), clause("c")]),
        ]);
        let simplified = expr.simplify();

        if let WhereExpr::And(children) = simplified {
            assert_eq!(children.len(), 3);
        } else {
            panic!("Expected And");
        }
    }

    #[test]
    fn test_demorgan_not_and() {
        let expr = WhereExpr::Not(Box::new(WhereExpr::And(vec![clause("a"), clause("b")])));
        let simplified = expr.simplify();
        if let WhereExpr::Or(children) = simplified {
            assert_eq!(children.len(), 2);
        } else {
            panic!("Expected Or");
        }
    }

    #[test]
    fn test_and_with_only_true() {
        let expr = WhereExpr::And(vec![WhereExpr::True, WhereExpr::True]);
        assert_eq!(expr.simplify(), WhereExpr::True);
    }

    #[test]
    fn test_or_with_only_false() {
        let expr = WhereExpr::Or(vec![WhereExpr::False, WhereExpr::False]);
        assert_eq!(expr.simplify(), WhereExpr::False);
    }

    #[test]
    fn test_demorgan_not_or() {
        let expr = WhereExpr::Not(Box::new(WhereExpr::Or(vec![clause("a"), clause("b")])));
        let simplified = expr.simplify();
        if let WhereExpr::And(children) = simplified {
            assert_eq!(children.len(), 2);
        } else {
            panic!("Expected And");
        }
    }

    #[test]
    fn test_double_negation_complex() {
        let inner = WhereExpr::Or(vec![clause("a"), clause("b")]);
        let expr = WhereExpr::Not(Box::new(WhereExpr::Not(Box::new(inner.clone()))));
        assert_eq!(expr.simplify(), inner);
    }

    #[test]
    fn test_not_clause() {
        let expr = WhereExpr::Not(Box::new(clause("foo")));
        let simplified = expr.simplify();
        match simplified {
            WhereExpr::Not(boxed) => {
                assert!(matches!(*boxed, WhereExpr::Clause(_)));
            }
            _ => panic!("Expected Not"),
        }
    }

    #[test]
    fn test_complex_nested_expression() {
        let expr = WhereExpr::Not(Box::new(WhereExpr::And(vec![
            WhereExpr::Or(vec![
                clause("a"),
                WhereExpr::False,
                WhereExpr::Not(Box::new(clause("b"))),
                WhereExpr::Or(vec![
                    clause("c"),
                    WhereExpr::True,
                    WhereExpr::Not(Box::new(WhereExpr::False)),
                ]),
            ]),
            WhereExpr::And(vec![
                clause("d"),
                WhereExpr::True,
                WhereExpr::Not(Box::new(WhereExpr::Or(vec![clause("e"), WhereExpr::False]))),
            ]),
            WhereExpr::Not(Box::new(WhereExpr::Not(Box::new(clause("f"))))),
        ])));

        let simplified = expr.simplify();

        fn contains_clause_f(expr: &WhereExpr) -> bool {
            match expr {
                WhereExpr::Clause(c) => c.field == "f",
                WhereExpr::And(children) | WhereExpr::Or(children) => {
                    children.iter().any(contains_clause_f)
                }
                WhereExpr::Not(inner) => contains_clause_f(inner),
                _ => false,
            }
        }

        assert!(
            matches!(simplified, WhereExpr::Or(_)),
            "Expected top-level Or"
        );
        assert!(
            contains_clause_f(&simplified),
            "Simplified expression must contain clause(\"f\")"
        );
    }
}
