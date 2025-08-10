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
        use Cmp::*;

        match cmp {
            Eq => actual.text_eq(expected, TextMode::Cs),
            Ne => actual.text_eq(expected, TextMode::Cs).map(|b| !b),
            Contains => actual.text_contains(expected, TextMode::Cs),
            StartsWith => actual.text_starts_with(expected, TextMode::Cs),
            EndsWith => actual.text_ends_with(expected, TextMode::Cs),

            // CI variants — ensure these exist in your Cmp enum
            EqCi => actual.text_eq(expected, TextMode::Ci),
            NeCi => actual.text_eq(expected, TextMode::Ci).map(|b| !b),
            ContainsCi => actual.text_contains(expected, TextMode::Ci),
            StartsWithCi => actual.text_starts_with(expected, TextMode::Ci),
            EndsWithCi => actual.text_ends_with(expected, TextMode::Ci),

            _ => None,
        }
    }

    /// Collection membership using Value helpers
    fn coerce_collection(actual: &Value, expected: &Value, cmp: Cmp) -> Option<bool> {
        use Cmp::*;

        match cmp {
            AllIn => actual.contains_all(expected),
            AnyIn => actual.contains_any(expected),
            Contains => actual.contains(expected),
            In => actual.in_list(expected),
            IsEmpty => actual.is_empty(),
            IsNotEmpty => actual.is_not_empty(),
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
