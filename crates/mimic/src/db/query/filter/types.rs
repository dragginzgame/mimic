use crate::db::query::{FilterDsl, FilterExpr};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Converts a typed filter struct into a `FilterExpr`.
///
pub trait IntoFilterExpr {
    fn into_expr(self, field: &str) -> Option<FilterExpr>;
}

///
/// ContainsFilter
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ContainsFilter {
    pub contains: Option<String>,
    pub all_in: Option<Vec<String>>,
    pub any_in: Option<Vec<String>>,
    pub is_empty: Option<bool>,

    /// Field does not contain the given value.
    pub not_contains: Option<String>,

    /// Field contains none of the given values (disjoint).
    pub not_any_in: Option<Vec<String>>,

    /// Field does not contain all of the given values (missing at least one).
    pub not_all_in: Option<Vec<String>>,
}

impl IntoFilterExpr for ContainsFilter {
    fn into_expr(self, field: &str) -> Option<FilterExpr> {
        let dsl = FilterDsl;
        let mut exprs = vec![];

        // positive variants
        if let Some(v) = self.contains {
            exprs.push(dsl.contains(field, v));
        }
        if let Some(vs) = self.any_in {
            exprs.push(dsl.any_in(field, vs));
        }
        if let Some(vs) = self.all_in {
            exprs.push(dsl.all_in(field, vs));
        }

        // negative variants
        if let Some(v) = self.not_contains {
            exprs.push(FilterExpr::Not(Box::new(dsl.contains(field, v))));
        }
        if let Some(vs) = self.not_any_in {
            exprs.push(FilterExpr::Not(Box::new(dsl.any_in(field, vs))));
        }
        if let Some(vs) = self.not_all_in {
            exprs.push(FilterExpr::Not(Box::new(dsl.all_in(field, vs))));
        }

        // emptiness
        if let Some(is_empty) = self.is_empty {
            if is_empty {
                exprs.push(dsl.is_empty(field));
            } else {
                exprs.push(dsl.is_not_empty(field));
            }
        }

        FilterDsl::all(exprs)
    }
}

///
/// EqualityFilter
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct EqualityFilter {
    pub eq: Option<String>,
    pub ne: Option<String>,
    pub in_: Option<Vec<String>>,
    pub not_in: Option<Vec<String>>,
    pub is_none: Option<bool>,
}

impl IntoFilterExpr for EqualityFilter {
    fn into_expr(self, field: &str) -> Option<FilterExpr> {
        let dsl = FilterDsl;
        let mut exprs = vec![];

        if let Some(v) = self.eq {
            exprs.push(dsl.eq(field, v));
        }
        if let Some(v) = self.ne {
            exprs.push(dsl.ne(field, v));
        }
        if let Some(vs) = self.in_ {
            exprs.push(dsl.in_iter(field, vs));
        }
        if let Some(vs) = self.not_in {
            exprs.push(dsl.not_in_iter(field, vs));
        }

        // some/none
        if let Some(is_none) = self.is_none {
            if is_none {
                exprs.push(dsl.is_none(field));
            } else {
                exprs.push(dsl.is_some(field));
            }
        }

        FilterDsl::all(exprs)
    }
}

///
/// RangeFilter
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct RangeFilter {
    pub gt: Option<i64>,
    pub gte: Option<i64>,
    pub lt: Option<i64>,
    pub lte: Option<i64>,
    pub between: Option<(i64, i64)>,
}

impl IntoFilterExpr for RangeFilter {
    fn into_expr(self, field: &str) -> Option<FilterExpr> {
        let dsl = FilterDsl;
        let mut exprs = vec![];

        if let Some(v) = self.gt {
            exprs.push(dsl.gt(field, v));
        }
        if let Some(v) = self.gte {
            exprs.push(dsl.gte(field, v));
        }
        if let Some(v) = self.lt {
            exprs.push(dsl.lt(field, v));
        }
        if let Some(v) = self.lte {
            exprs.push(dsl.lte(field, v));
        }
        if let Some((min, max)) = self.between {
            exprs.push(dsl.gte(field, min));
            exprs.push(dsl.lte(field, max));
        }

        FilterDsl::all(exprs)
    }
}

///
/// TextFilter
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextFilter {
    pub eq: Option<String>,
    pub eq_ci: Option<String>,
    pub ne: Option<String>,
    pub ne_ci: Option<String>,
    pub contains: Option<String>,
    pub contains_ci: Option<String>,
    pub starts_with: Option<String>,
    pub starts_with_ci: Option<String>,
    pub ends_with: Option<String>,
    pub ends_with_ci: Option<String>,
    pub is_empty: Option<bool>,
}

impl IntoFilterExpr for TextFilter {
    fn into_expr(self, field: &str) -> Option<FilterExpr> {
        let dsl = FilterDsl;
        let mut exprs = vec![];

        // equality
        if let Some(v) = self.eq {
            exprs.push(dsl.eq(field, v));
        }
        if let Some(v) = self.eq_ci {
            exprs.push(dsl.eq_ci(field, v));
        }
        if let Some(v) = self.ne {
            exprs.push(dsl.ne(field, v));
        }
        if let Some(v) = self.ne_ci {
            exprs.push(dsl.ne_ci(field, v));
        }

        // contains / substring
        if let Some(v) = self.contains {
            exprs.push(dsl.contains(field, v));
        }
        if let Some(v) = self.contains_ci {
            exprs.push(dsl.contains_ci(field, v));
        }

        // prefix / suffix
        if let Some(v) = self.starts_with {
            exprs.push(dsl.starts_with(field, v));
        }
        if let Some(v) = self.starts_with_ci {
            exprs.push(dsl.starts_with_ci(field, v));
        }
        if let Some(v) = self.ends_with {
            exprs.push(dsl.ends_with(field, v));
        }
        if let Some(v) = self.ends_with_ci {
            exprs.push(dsl.ends_with_ci(field, v));
        }

        // emptiness
        if let Some(is_empty) = self.is_empty {
            if is_empty {
                exprs.push(dsl.is_empty(field));
            } else {
                exprs.push(dsl.is_not_empty(field));
            }
        }

        if exprs.is_empty() {
            None
        } else {
            FilterDsl::all(exprs)
        }
    }
}
