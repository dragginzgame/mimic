use crate::{
    core::{
        traits::{EntityKind, FieldValues},
        value::Value,
    },
    db::query::{Cmp, FilterClause, FilterExpr, QueryError, QueryValidate},
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
        // Try numeric/structural coercions first
        if let Some(res) = Self::coerce_match(left, right, cmp) {
            return res;
        }

        // Try text-based coercion
        if let Some(res) = Self::coerce_text_match(left, right, cmp) {
            return res;
        }

        // Then collection contains
        if let Some(res) = Self::coerce_collection_contains(left, right, cmp) {
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

            _ => None,
        }
    }

    /// Applies a case-insensitive textual comparison if both values can be viewed as strings.
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

    fn coerce_collection_contains(actual: &Value, expected: &Value, cmp: Cmp) -> Option<bool> {
        match (actual, expected) {
            // check if list contains the scalar value
            (Value::List(items), item) => match cmp {
                Cmp::Contains => Some(items.iter().any(|elem| elem.as_ref() == item)),
                _ => None,
            },

            // optional future: support "scalar contains list" (e.g., "abc" contains ["a", "b"])
            _ => None,
        }
    }
}

impl<E: EntityKind> QueryValidate<E> for FilterExpr {
    fn validate(&self) -> Result<(), QueryError> {
        match self {
            Self::True | Self::False => Ok(()),

            Self::Clause(c) => {
                if !E::FIELDS.contains(&c.field.as_str()) {
                    return Err(QueryError::InvalidFilterField(c.field.clone()));
                }

                // (Optional) type checking could happen here
                Ok(())
            }

            Self::And(children) | Self::Or(children) => {
                for expr in children {
                    QueryValidate::<E>::validate(expr)?;
                }
                Ok(())
            }

            Self::Not(inner) => QueryValidate::<E>::validate(inner),
        }
    }
}
