use crate::{
    core::{traits::FieldValues, value::Value},
    db::query::{Cmp, FilterClause, FilterExpr},
};

///
/// FilterEvaluator
///

pub struct FilterEvaluator<'a> {
    values: &'a dyn FieldValues,
}

impl<'a> FilterEvaluator<'a> {
    #[must_use]
    pub const fn new(values: &'a dyn FieldValues) -> Self {
        Self { values }
    }

    #[must_use]
    pub fn eval(&self, expr: &FilterExpr) -> bool {
        match expr {
            FilterExpr::True => true,
            FilterExpr::False => false,
            FilterExpr::Clause(clause) => self.eval_clause(clause),
            FilterExpr::And(children) => children.iter().all(|e| self.eval(e)),
            FilterExpr::Or(children) => children.iter().any(|e| self.eval(e)),
            FilterExpr::Not(inner) => !self.eval(inner),
        }
    }

    // eval_clause
    fn eval_clause(&self, clause: &FilterClause) -> bool {
        self.values
            .get_value(clause.field.as_str())
            .is_some_and(|actual| Self::compare(&actual, clause.cmp, &clause.value))
    }

    // compare
    fn compare(left: &Value, cmp: Cmp, right: &Value) -> bool {
        // 1. Try numeric/structural coercions first
        if let Some(res) = Self::coerce_match(left, right, cmp) {
            return res;
        }

        // 2. Try text-based coercion
        if let Some(res) = Self::coerce_text_match(left, right, cmp) {
            return res;
        }

        // 3. Fall back to strict comparison
        match cmp {
            // values
            Cmp::Eq => left == right,
            Cmp::Ne => left != right,
            Cmp::Lt => left < right,
            Cmp::Lte => left <= right,
            Cmp::Gt => left > right,
            Cmp::Gte => left >= right,

            // lists
            Cmp::In => match right {
                Value::List(items) => items.iter().any(|v| v.as_ref() == left),
                _ => false,
            },

            Cmp::AllIn => match (left, right) {
                (Value::List(left_items), Value::List(right_items)) => right_items
                    .iter()
                    .all(|r| left_items.iter().any(|l| l == r)),
                _ => false,
            },

            Cmp::AnyIn => match (left, right) {
                (Value::List(left_items), Value::List(right_items)) => right_items
                    .iter()
                    .any(|r| left_items.iter().any(|l| l == r)),
                _ => false,
            },

            _ => false, // should only be text ops here, already handled
        }
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    fn coerce_match(actual: &Value, expected: &Value, cmp: Cmp) -> Option<bool> {
        match (actual, expected) {
            // Int ↔ Nat
            (Value::Int(a), Value::Nat(b)) => {
                if *a < 0 {
                    Some(matches!(cmp, Cmp::Ne)) // negative can't equal unsigned
                } else {
                    Some(cmp.compare_order((*a as u64).cmp(b)))
                }
            }
            (Value::Nat(a), Value::Int(b)) => {
                if *b < 0 {
                    Some(matches!(cmp, Cmp::Ne))
                } else {
                    Some(cmp.compare_order(a.cmp(&(*b as u64))))
                }
            }

            // Int ↔ Float
            (Value::Int(a), Value::Float(b)) => {
                Some(cmp.compare_order((*a as f64).partial_cmp(b)?))
            }
            (Value::Float(a), Value::Int(b)) => {
                Some(cmp.compare_order(a.partial_cmp(&(*b as f64))?))
            }

            // Nat ↔ Float
            (Value::Nat(a), Value::Float(b)) => {
                Some(cmp.compare_order((*a as f64).partial_cmp(b)?))
            }
            (Value::Float(a), Value::Nat(b)) => {
                Some(cmp.compare_order(a.partial_cmp(&(*b as f64))?))
            }

            // Ulid ↔ Text
            (Value::Ulid(a), Value::Text(b)) => Some(cmp.compare_order(a.to_string().cmp(b))),
            (Value::Text(a), Value::Ulid(b)) => Some(cmp.compare_order(a.cmp(&b.to_string()))),

            // Principal ↔ Text
            (Value::Principal(a), Value::Text(b)) => Some(cmp.compare_order(a.to_text().cmp(b))),
            (Value::Text(a), Value::Principal(b)) => Some(cmp.compare_order(a.cmp(&b.to_text()))),

            _ => None,
        }
    }

    /// Applies a case-insensitive textual comparison if both values can be viewed as strings.
    #[must_use]
    fn coerce_text_match(actual: &Value, expected: &Value, cmp: Cmp) -> Option<bool> {
        let a = actual.to_searchable_string()?;
        let b = expected.to_searchable_string()?;

        let a = a.to_lowercase();
        let b = b.to_lowercase();

        Some(match cmp {
            Cmp::Contains => a.contains(&b),
            Cmp::StartsWith => a.starts_with(&b),
            Cmp::EndsWith => a.ends_with(&b),

            _ => return None,
        })
    }
}
