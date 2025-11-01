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
    pub is_not_empty: Option<bool>,
}

impl IntoFilterExpr for ContainsFilter {
    fn into_expr(self, field: &str) -> Option<FilterExpr> {
        let dsl = FilterDsl;
        let mut exprs = vec![];

        if let Some(v) = self.contains {
            exprs.push(dsl.contains(field, v));
        }
        if let Some(vs) = self.any_in {
            exprs.push(dsl.any_in(field, vs));
        }
        if let Some(vs) = self.all_in {
            exprs.push(dsl.all_in(field, vs));
        }

        // is_empty
        if self.is_empty == Some(true) {
            exprs.push(dsl.is_empty(field));
        } else if self.is_not_empty == Some(true) {
            exprs.push(dsl.is_not_empty(field));
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
    pub is_some: Option<bool>,
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
        if self.is_none == Some(true) {
            exprs.push(dsl.is_none(field));
        } else if self.is_some == Some(true) {
            exprs.push(dsl.is_some(field));
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

        FilterDsl::all(exprs)
    }
}

///
/// TextFilter
///
/// case_insensitive flag could be made better... currently we
/// lose the ability to mix case-sensitive and case-insensitive filters
/// in one request (e.g., eq_ci and contains simultaneously).
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextFilter {
    pub eq: Option<String>,
    pub ne: Option<String>,
    pub contains: Option<String>,
    pub starts_with: Option<String>,
    pub ends_with: Option<String>,
    /// When true, comparisons should be case-insensitive.
    pub case_insensitive: bool,

    pub is_empty: Option<bool>,
    pub is_not_empty: Option<bool>,
}

impl IntoFilterExpr for TextFilter {
    fn into_expr(self, field: &str) -> Option<FilterExpr> {
        let dsl = FilterDsl;

        let mut exprs = vec![];

        // equality
        if let Some(v) = self.eq {
            exprs.push(if self.case_insensitive {
                dsl.eq_ci(field, v)
            } else {
                dsl.eq(field, v)
            });
        }
        if let Some(v) = self.ne {
            exprs.push(if self.case_insensitive {
                dsl.ne_ci(field, v)
            } else {
                dsl.ne(field, v)
            });
        }

        // pattern matching
        if let Some(v) = self.contains {
            exprs.push(if self.case_insensitive {
                dsl.contains_ci(field, v)
            } else {
                dsl.contains(field, v)
            });
        }
        if let Some(v) = self.starts_with {
            exprs.push(if self.case_insensitive {
                dsl.starts_with_ci(field, v)
            } else {
                dsl.starts_with(field, v)
            });
        }
        if let Some(v) = self.ends_with {
            exprs.push(if self.case_insensitive {
                dsl.ends_with_ci(field, v)
            } else {
                dsl.ends_with(field, v)
            });
        }

        // empty / non-empty
        if self.is_empty == Some(true) {
            exprs.push(dsl.is_empty(field));
        } else if self.is_not_empty == Some(true) {
            exprs.push(dsl.is_not_empty(field));
        }

        FilterDsl::all(exprs)
    }
}
