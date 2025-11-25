use crate::{
    core::{
        traits::{EntityKind, FieldValues},
        value::{TextMode, Value},
    },
    db::{
        primitives::{Cmp, FilterClause, FilterExpr},
        query::{QueryError, QueryValidate},
    },
    types::Ulid,
};
use std::{cmp::Ordering, convert::TryFrom};

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
        ) {
            if let Some(ord) = left.cmp_numeric(right) {
                return cmp.compare_order(ord);
            }

            // Allow range comparators to treat collection length as the numeric value.
            if matches!(cmp, Cmp::Lt | Cmp::Lte | Cmp::Gt | Cmp::Gte)
                && let Some(ord) = Self::cmp_collection_len(left, right)
            {
                return cmp.compare_order(ord);
            }
        }

        // 2) Text ops (CS/CI explicit)
        if let Some(res) = Self::coerce_text_match(left, right, cmp) {
            return res;
        }

        // 2b) Enum ops
        if let Some(res) = Self::coerce_enum(left, right, cmp) {
            return res;
        }

        // 2c) ULID vs Text equality (string-based comparison)
        if let Some(res) = Self::coerce_ulid_text(left, right, cmp) {
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

            // Negated membership
            Cmp::NotIn => actual.in_list(expected).map(|v| !v),

            // CI variants
            Cmp::AllInCi => actual.contains_all_ci(expected),
            Cmp::AnyInCi => actual.contains_any_ci(expected),
            Cmp::InCi => actual.in_list_ci(expected),

            Cmp::IsEmpty => actual.is_empty(),
            Cmp::IsNotEmpty => actual.is_not_empty(),
            _ => None,
        }
    }

    /// Enum checking
    fn coerce_enum(left: &Value, right: &Value, cmp: Cmp) -> Option<bool> {
        match (left, right) {
            (Value::Enum(l), Value::Enum(r)) if l.path == r.path => {
                match cmp {
                    Cmp::Eq => Some(l.variant == r.variant),
                    Cmp::Ne => Some(l.variant != r.variant),

                    _ => None, // no string ops here
                }
            }
            _ => None,
        }
    }

    fn coerce_ulid_text(left: &Value, right: &Value, cmp: Cmp) -> Option<bool> {
        match (left, right, cmp) {
            (Value::Ulid(lhs), Value::Text(rhs), Cmp::Eq | Cmp::Ne)
            | (Value::Text(rhs), Value::Ulid(lhs), Cmp::Eq | Cmp::Ne) => {
                let parsed = Ulid::from_str(rhs).ok();

                parsed.map(|rhs_ulid| match cmp {
                    Cmp::Eq => rhs_ulid == *lhs,
                    Cmp::Ne => rhs_ulid != *lhs,
                    _ => unreachable!("handled in match"),
                })
            }
            _ => None,
        }
    }

    fn cmp_collection_len(left: &Value, right: &Value) -> Option<Ordering> {
        match left {
            Value::List(items) => {
                let len = i64::try_from(items.len()).ok()?;
                Value::Int(len).cmp_numeric(right)
            }
            _ => None,
        }
    }
}

impl<E: EntityKind> QueryValidate<E> for FilterExpr {
    #[allow(clippy::match_same_arms, clippy::too_many_lines)]
    fn validate(&self) -> Result<(), QueryError> {
        match self {
            Self::True | Self::False => Ok(()),

            Self::Clause(c) => {
                if !E::FIELDS.contains(&c.field.as_str()) {
                    return Err(QueryError::InvalidFilterField(c.field.clone()));
                }

                let v = &c.value;
                let field = &c.field;

                match c.cmp {
                    // Ordering comparators
                    Cmp::Lt | Cmp::Lte | Cmp::Gt | Cmp::Gte => {
                        if !(v.is_numeric() || v.is_text()) {
                            return Err(QueryError::InvalidFilterValue(format!(
                                "field '{field}' expects comparable RHS (numeric or text) for {cmp:?}",
                                cmp = c.cmp
                            )));
                        }
                    }

                    // Case-sensitive text
                    Cmp::StartsWith | Cmp::EndsWith | Cmp::Contains => {
                        if !v.is_text() {
                            // Allow non-text only for Contains (collection)
                            if !matches!(c.cmp, Cmp::Contains) {
                                return Err(QueryError::InvalidFilterValue(format!(
                                    "field '{field}' expects text RHS for {cmp:?}",
                                    cmp = c.cmp
                                )));
                            }
                        }
                    }

                    // Case-insensitive text
                    Cmp::EqCi
                    | Cmp::NeCi
                    | Cmp::ContainsCi
                    | Cmp::StartsWithCi
                    | Cmp::EndsWithCi => {
                        if !v.is_text() {
                            return Err(QueryError::InvalidFilterValue(format!(
                                "field '{field}' expects text RHS for {cmp:?}",
                                cmp = c.cmp
                            )));
                        }
                    }

                    // Null / presence
                    Cmp::IsSome | Cmp::IsNone | Cmp::IsEmpty | Cmp::IsNotEmpty => {
                        if !v.is_unit() {
                            return Err(QueryError::InvalidFilterValue(format!(
                                "field '{field}' expects unit RHS for {cmp:?}",
                                cmp = c.cmp
                            )));
                        }
                    }

                    // Membership & equality family
                    Cmp::In | Cmp::NotIn | Cmp::Eq | Cmp::Ne => {
                        // no strong type enforcement — allow scalar or list
                    }

                    // Collection membership
                    Cmp::AnyIn | Cmp::AllIn => {
                        // Allow list RHS; tolerate scalar
                        if !matches!(v, Value::List(_)) {
                            // scalar fallback allowed
                        }
                    }

                    // Case-insensitive collection membership
                    Cmp::AnyInCi | Cmp::AllInCi | Cmp::InCi => match v {
                        Value::List(items) => {
                            if !items.iter().all(Value::is_text) {
                                return Err(QueryError::InvalidFilterValue(format!(
                                    "field '{field}' {cmp:?} expects list of text",
                                    cmp = c.cmp
                                )));
                            }
                        }
                        other => {
                            if !other.is_text() {
                                return Err(QueryError::InvalidFilterValue(format!(
                                    "field '{field}' {cmp:?} expects text RHS",
                                    cmp = c.cmp
                                )));
                            }
                        }
                    },

                    // -------------------------
                    // MAP FILTERS
                    // -------------------------
                    Cmp::MapContainsKey | Cmp::MapNotContainsKey => {
                        if !v.is_scalar() {
                            return Err(QueryError::InvalidFilterValue(format!(
                                "field '{field}' expects scalar key for {cmp:?}",
                                cmp = c.cmp
                            )));
                        }
                    }

                    Cmp::MapContainsValue | Cmp::MapNotContainsValue => {
                        // Any Value allowed as map values
                    }

                    Cmp::MapContainsEntry | Cmp::MapNotContainsEntry => {
                        match v {
                            Value::List(pair) if pair.len() == 2 => {
                                // Key must be scalar
                                if !pair[0].is_scalar() {
                                    return Err(QueryError::InvalidFilterValue(format!(
                                        "field '{field}' expects scalar key in entry pair for {cmp:?}",
                                        cmp = c.cmp
                                    )));
                                }
                                // Value can be any Value
                            }
                            _ => {
                                return Err(QueryError::InvalidFilterValue(format!(
                                    "field '{field}' expects (key, value) pair for {cmp:?}",
                                    cmp = c.cmp
                                )));
                            }
                        }
                    }
                }

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
