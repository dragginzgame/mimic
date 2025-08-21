use crate::{
    core::{
        traits::{EntityKind, FieldValues},
        value::{TextMode, Value},
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

    /// Core comparator dispatch
    fn compare(left: &Value, cmp: Cmp, right: &Value) -> bool {
        // 0) Presence/null checks (RHS ignored)
        match cmp {
            Cmp::IsNone => return matches!(left, Value::None),
            Cmp::IsSome => return !matches!(left, Value::None),
            _ => {}
        }

        // 1) Numeric ordering (Decimal-first, then f64-safe handled inside Value::cmp_numeric)
        if matches!(
            cmp,
            Cmp::Eq | Cmp::Ne | Cmp::Lt | Cmp::Lte | Cmp::Gt | Cmp::Gte
        ) && let Some(ord) = left.cmp_numeric(right)
        {
            return cmp.compare_order(ord);
        }

        // 2) Text ops (CS/CI explicit)
        if let Some(res) = Self::coerce_text_match(left, right, cmp) {
            return res;
        }

        // 3) Collection membership ops
        if let Some(res) = Self::coerce_collection(left, right, cmp) {
            return res;
        }

        // 4) final fallback: strict same-variant compare only
        match cmp {
            Cmp::Eq => left == right,
            Cmp::Ne => left != right,
            Cmp::Lt => left < right,
            Cmp::Lte => left <= right,
            Cmp::Gt => left > right,
            Cmp::Gte => left >= right,
            _ => false,
        }
    }

    /// Text dispatch (maps Cmp → Value::text_* with CS/CI)
    fn coerce_text_match(actual: &Value, expected: &Value, cmp: Cmp) -> Option<bool> {
        match cmp {
            Cmp::Eq => actual.text_eq(expected, TextMode::Cs),
            Cmp::Ne => actual.text_eq(expected, TextMode::Cs).map(|b| !b),
            Cmp::Contains => actual.text_contains(expected, TextMode::Cs),
            Cmp::StartsWith => actual.text_starts_with(expected, TextMode::Cs),
            Cmp::EndsWith => actual.text_ends_with(expected, TextMode::Cs),

            // CI variants — ensure these exist in your Cmp enum
            Cmp::EqCi => actual.text_eq(expected, TextMode::Ci),
            Cmp::NeCi => actual.text_eq(expected, TextMode::Ci).map(|b| !b),
            Cmp::ContainsCi => actual.text_contains(expected, TextMode::Ci),
            Cmp::StartsWithCi => actual.text_starts_with(expected, TextMode::Ci),
            Cmp::EndsWithCi => actual.text_ends_with(expected, TextMode::Ci),

            _ => None,
        }
    }

    /// Collection membership using Value helpers
    fn coerce_collection(actual: &Value, expected: &Value, cmp: Cmp) -> Option<bool> {
        match cmp {
            Cmp::AllIn => actual.contains_all(expected),
            Cmp::AnyIn => actual.contains_any(expected),
            Cmp::Contains => actual.contains(expected),
            Cmp::In => actual.in_list(expected),

            // CI variants
            Cmp::AllInCi => actual.contains_all_ci(expected),
            Cmp::AnyInCi => actual.contains_any_ci(expected),
            Cmp::InCi => actual.in_list_ci(expected),

            Cmp::IsEmpty => actual.is_empty(),
            Cmp::IsNotEmpty => actual.is_not_empty(),
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
