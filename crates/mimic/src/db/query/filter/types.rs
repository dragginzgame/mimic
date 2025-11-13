use crate::db::query::{FilterDsl, FilterExpr};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Filter
///

pub trait Filter {
    type Payload: IntoFilterExpr;

    /// Converts a payload into a FilterExpr for this filter.
    fn to_expr(payload: Self::Payload) -> FilterExpr {
        payload.into_expr()
    }
}

///
/// IntoFilterExpr
///

pub trait IntoFilterExpr {
    fn into_expr(self) -> FilterExpr;
}

pub trait IntoScopedFilterExpr {
    fn into_scoped(self, path: &str) -> FilterExpr;
}

///
/// NoFilter
/// (#nofilter)
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct NoFilter;

impl IntoFilterExpr for NoFilter {
    fn into_expr(self) -> FilterExpr {
        FilterExpr::True
    }
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
    fn into_expr(self, path: Option<&str>) -> FilterExpr {
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
    fn into_expr(self, path: Option<&str>) -> FilterExpr {
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

impl IntoFieldFilterExpr for RangeFilter {
    fn into_field_expr(self, field: &str) -> FilterExpr {
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
/// SetFilter
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetFilter {
    /// Filter for membership checks (e.g., contains / any_in / all_in)
    pub contains: Option<ContainsFilter>,

    /// Optional equality-like filter (for entire set equality)
    pub eq: Option<EqualityFilter>,

    /// Optional range-like filter (for cardinality / size)
    pub len: Option<RangeFilter>,
}

impl IntoFilterExpr for SetFilter {
    fn into_expr(self) -> FilterExpr {
        let mut exprs = Vec::new();

        if let Some(f) = self.contains {
            exprs.push(f.into_field_expr("value"));
        }
        if let Some(f) = self.len {
            exprs.push(f.into_field_expr("len"));
        }

        FilterDsl::all(exprs)
    }
}

///
/// TextFilter
///

pub struct TextFilter;

impl Filter for TextFilter {
    type Payload = TextFilterPayload;
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct TextFilterPayload {
    pub actions: Vec<TextFilterAction>,
    pub is_empty: Option<bool>,
}

///
/// TextFilterAction
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct TextFilterAction {
    pub op: TextFilterOp,
    pub case_insensitive: bool,
    pub values: Vec<String>,
}

///
/// TextFilterOp
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub enum TextFilterOp {
    Equal,
    NotEqual,
    Contains,
    StartsWith,
    EndsWith,
}

impl IntoFilterExpr for TextFilterPayload {
    fn into_expr(self) -> FilterExpr {
        let dsl = FilterDsl;
        let mut exprs = vec![];

        for action in self.actions {
            let or_exprs = action
                .values
                .into_iter()
                .map(|v| match (action.op.clone(), action.case_insensitive) {
                    (TextFilterOp::Equal, false) => dsl.eq(field, v),
                    (TextFilterOp::Equal, true) => dsl.eq_ci(field, v),
                    (TextFilterOp::NotEqual, false) => dsl.ne(field, v),
                    (TextFilterOp::NotEqual, true) => dsl.ne_ci(field, v),
                    (TextFilterOp::Contains, false) => dsl.contains(field, v),
                    (TextFilterOp::Contains, true) => dsl.contains_ci(field, v),
                    (TextFilterOp::StartsWith, false) => dsl.starts_with(field, v),
                    (TextFilterOp::StartsWith, true) => dsl.starts_with_ci(field, v),
                    (TextFilterOp::EndsWith, false) => dsl.ends_with(field, v),
                    (TextFilterOp::EndsWith, true) => dsl.ends_with_ci(field, v),
                })
                .collect::<Vec<_>>();

            exprs.push(FilterDsl::any(or_exprs));
        }

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
