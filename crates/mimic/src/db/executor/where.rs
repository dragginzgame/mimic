use crate::{
    db::query::{Comparator, WhereClause, WhereExpr},
    ops::{Value, Values, traits::EntityKind},
};
use std::marker::PhantomData;

///
/// WhereEvaluator
///
pub struct WhereEvaluator<E: EntityKind>(PhantomData<E>);

impl<E> WhereEvaluator<E>
where
    E: EntityKind,
{
    pub fn eval(expr: &WhereExpr, entity: &E) -> bool {
        // Get all fields once, as a HashMap for quick lookup
        let cached_values = entity.values();

        Self::eval_cached(expr, &cached_values)
    }

    fn eval_cached(expr: &WhereExpr, values: &Values) -> bool {
        match expr {
            WhereExpr::Clause(clause) => Self::eval_clause(clause, values),
            WhereExpr::And(children) => children.iter().all(|c| Self::eval_cached(c, values)),
            WhereExpr::Or(children) => children.iter().any(|c| Self::eval_cached(c, values)),
            WhereExpr::Not(child) => !Self::eval_cached(child, values),
        }
    }

    fn eval_clause(clause: &WhereClause, values: &Values) -> bool {
        if let Some(actual) = values.get(clause.field.as_str()) {
            Self::compare(actual, &clause.cmp, &clause.value)
        } else {
            false
        }
    }

    fn compare(actual: &Value, cmp: &Comparator, expected: &Value) -> bool {
        match cmp {
            Comparator::Eq => actual == expected,
            Comparator::Ne => actual != expected,
            Comparator::Lt => actual < expected,
            Comparator::Ltoe => actual <= expected,
            Comparator::Gt => actual > expected,
            Comparator::Gtoe => actual >= expected,
        }
    }
}
