use crate::{
    core::value::{Value, Values},
    db::query::{Cmp, WhereClause, WhereExpr},
};

///
/// WhereEvaluator
///

pub struct WhereEvaluator<'a> {
    values: &'a Values,
}

impl<'a> WhereEvaluator<'a> {
    #[must_use]
    pub fn new(values: &'a Values) -> Self {
        Self { values }
    }

    #[must_use]
    pub fn eval(&self, expr: &WhereExpr) -> bool {
        match expr {
            WhereExpr::True => true,
            WhereExpr::False => false,
            WhereExpr::Clause(clause) => self.eval_clause(clause),
            WhereExpr::And(children) => children.iter().all(|e| self.eval(e)),
            WhereExpr::Or(children) => children.iter().any(|e| self.eval(e)),
            WhereExpr::Not(inner) => !self.eval(inner),
        }
    }

    fn eval_clause(&self, clause: &WhereClause) -> bool {
        self.values
            .get(&clause.field.as_str())
            .map(|actual| Self::compare(actual, &clause.cmp, &clause.value))
            .unwrap_or(false)
    }

    fn compare(actual: &Value, cmp: &Cmp, expected: &Value) -> bool {
        match cmp {
            // general comparison
            Cmp::Eq => actual == expected,
            Cmp::Ne => actual != expected,
            Cmp::Lt => actual < expected,
            Cmp::Ltoe => actual <= expected,
            Cmp::Gt => actual > expected,
            Cmp::Gtoe => actual >= expected,

            // string matching
            Cmp::Contains => match (actual, expected) {
                (Value::Text(a), Value::Text(b)) => a.contains(b),
                _ => false,
            },

            Cmp::StartsWith => match (actual, expected) {
                (Value::Text(a), Value::Text(b)) => a.starts_with(b),
                _ => false,
            },

            Cmp::EndsWith => match (actual, expected) {
                (Value::Text(a), Value::Text(b)) => a.ends_with(b),
                _ => false,
            },
        }
    }
}
