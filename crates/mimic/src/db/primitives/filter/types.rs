use crate::db::primitives::filter::{FilterDsl, FilterExpr};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// FilterKind
///

pub trait FilterKind {
    type Payload: IntoScopedFilterExpr;

    fn to_expr(payload: Self::Payload, path: &str) -> FilterExpr {
        payload.into_scoped(path)
    }
}

///
/// IntoFilterExpr
/// Root-level: combines many field filters into one expression
///

pub trait IntoFilterExpr {
    fn into_expr(self) -> FilterExpr;
}

///
/// IntoScopedFilterExpr
/// Scoped-level: payloads and nested filters need the field path
///

pub trait IntoScopedFilterExpr {
    fn into_scoped(self, path: &str) -> FilterExpr;
}

///
/// NoFilterKind
///

pub struct NoFilterKind;

impl FilterKind for NoFilterKind {
    type Payload = NoFilter;
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

impl IntoScopedFilterExpr for NoFilter {
    fn into_scoped(self, _path: &str) -> FilterExpr {
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

impl ContainsFilter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn contains(mut self, value: impl Into<String>) -> Self {
        self.contains = Some(value.into());
        self
    }

    #[must_use]
    pub fn not_contains(mut self, value: impl Into<String>) -> Self {
        self.not_contains = Some(value.into());
        self
    }

    #[must_use]
    pub fn any_in<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self::extend_slot(&mut self.any_in, values);
        self
    }

    #[must_use]
    pub fn all_in<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self::extend_slot(&mut self.all_in, values);
        self
    }

    #[must_use]
    pub fn not_any_in<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self::extend_slot(&mut self.not_any_in, values);
        self
    }

    #[must_use]
    pub fn not_all_in<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self::extend_slot(&mut self.not_all_in, values);
        self
    }

    #[must_use]
    pub const fn is_empty(mut self, val: bool) -> Self {
        self.is_empty = Some(val);
        self
    }

    #[must_use]
    pub const fn empty(self) -> Self {
        self.is_empty(true)
    }

    #[must_use]
    pub const fn not_empty(self) -> Self {
        self.is_empty(false)
    }

    fn extend_slot<I, T>(slot: &mut Option<Vec<String>>, values: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        slot.get_or_insert_with(Vec::new)
            .extend(values.into_iter().map(Into::into));
    }
}

impl IntoScopedFilterExpr for ContainsFilter {
    fn into_scoped(self, path: &str) -> FilterExpr {
        let dsl = FilterDsl;
        let mut exprs = vec![];

        // positive variants
        if let Some(v) = self.contains {
            exprs.push(dsl.contains(path, v));
        }
        if let Some(vs) = self.any_in {
            exprs.push(dsl.any_in(path, vs));
        }
        if let Some(vs) = self.all_in {
            exprs.push(dsl.all_in(path, vs));
        }

        // negative variants
        if let Some(v) = self.not_contains {
            exprs.push(FilterExpr::Not(Box::new(dsl.contains(path, v))));
        }
        if let Some(vs) = self.not_any_in {
            exprs.push(FilterExpr::Not(Box::new(dsl.any_in(path, vs))));
        }
        if let Some(vs) = self.not_all_in {
            exprs.push(FilterExpr::Not(Box::new(dsl.all_in(path, vs))));
        }

        // emptiness
        if let Some(is_empty) = self.is_empty {
            exprs.push(if is_empty {
                dsl.is_empty(path)
            } else {
                dsl.is_not_empty(path)
            });
        }

        FilterDsl::all(exprs)
    }
}

///
/// EqualityFilterKind
///

pub struct EqualityFilterKind;

impl FilterKind for EqualityFilterKind {
    type Payload = EqualityFilter;
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

impl EqualityFilter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn eq(mut self, value: impl Into<String>) -> Self {
        self.eq = Some(value.into());
        self
    }

    #[must_use]
    pub fn ne(mut self, value: impl Into<String>) -> Self {
        self.ne = Some(value.into());
        self
    }

    #[must_use]
    pub fn in_<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self::extend_slot(&mut self.in_, values);
        self
    }

    #[must_use]
    pub fn not_in<I, T>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self::extend_slot(&mut self.not_in, values);
        self
    }

    #[must_use]
    pub const fn is_none(mut self, val: bool) -> Self {
        self.is_none = Some(val);
        self
    }

    #[must_use]
    pub const fn none(self) -> Self {
        self.is_none(true)
    }

    #[must_use]
    pub const fn some(self) -> Self {
        self.is_none(false)
    }

    fn extend_slot<I, T>(slot: &mut Option<Vec<String>>, values: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        slot.get_or_insert_with(Vec::new)
            .extend(values.into_iter().map(Into::into));
    }
}

impl IntoScopedFilterExpr for EqualityFilter {
    fn into_scoped(self, field: &str) -> FilterExpr {
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

        if let Some(is_none) = self.is_none {
            exprs.push(if is_none {
                dsl.is_none(field)
            } else {
                dsl.is_some(field)
            });
        }

        FilterDsl::all(exprs)
    }
}

///
/// RangeFilterKind
///

pub struct RangeFilterKind;

impl FilterKind for RangeFilterKind {
    type Payload = RangeFilter;
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

impl RangeFilter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn gt(mut self, value: impl Into<i64>) -> Self {
        self.gt = Some(value.into());
        self
    }

    #[must_use]
    pub fn gte(mut self, value: impl Into<i64>) -> Self {
        self.gte = Some(value.into());
        self
    }

    #[must_use]
    pub fn lt(mut self, value: impl Into<i64>) -> Self {
        self.lt = Some(value.into());
        self
    }

    #[must_use]
    pub fn lte(mut self, value: impl Into<i64>) -> Self {
        self.lte = Some(value.into());
        self
    }

    #[must_use]
    pub fn between<M, N>(mut self, min: M, max: N) -> Self
    where
        M: Into<i64>,
        N: Into<i64>,
    {
        self.between = Some((min.into(), max.into()));
        self
    }
}

impl IntoScopedFilterExpr for RangeFilter {
    fn into_scoped(self, path: &str) -> FilterExpr {
        let dsl = FilterDsl;
        let mut exprs = vec![];

        if let Some(v) = self.gt {
            exprs.push(dsl.gt(path, v));
        }
        if let Some(v) = self.gte {
            exprs.push(dsl.gte(path, v));
        }
        if let Some(v) = self.lt {
            exprs.push(dsl.lt(path, v));
        }
        if let Some(v) = self.lte {
            exprs.push(dsl.lte(path, v));
        }
        if let Some((min, max)) = self.between {
            exprs.push(dsl.gte(path, min));
            exprs.push(dsl.lte(path, max));
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

impl SetFilter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn contains(mut self, f: impl FnOnce(ContainsFilter) -> ContainsFilter) -> Self {
        let filter = f(self.contains.unwrap_or_default());
        self.contains = Some(filter);
        self
    }

    #[must_use]
    pub fn contains_filter(mut self, filter: ContainsFilter) -> Self {
        self.contains = Some(filter);
        self
    }

    #[must_use]
    pub fn eq(mut self, f: impl FnOnce(EqualityFilter) -> EqualityFilter) -> Self {
        let filter = f(self.eq.unwrap_or_default());
        self.eq = Some(filter);
        self
    }

    #[must_use]
    pub fn eq_filter(mut self, filter: EqualityFilter) -> Self {
        self.eq = Some(filter);
        self
    }

    #[must_use]
    pub fn len(mut self, f: impl FnOnce(RangeFilter) -> RangeFilter) -> Self {
        let filter = f(self.len.unwrap_or_default());
        self.len = Some(filter);
        self
    }

    #[must_use]
    pub const fn len_filter(mut self, filter: RangeFilter) -> Self {
        self.len = Some(filter);
        self
    }
}

impl IntoScopedFilterExpr for SetFilter {
    fn into_scoped(self, field: &str) -> FilterExpr {
        let mut exprs = Vec::new();

        if let Some(f) = self.contains {
            // These all apply to the same field
            exprs.push(f.into_scoped(field));
        }

        if let Some(f) = self.eq {
            // Equality of the whole set, still same field
            exprs.push(f.into_scoped(field));
        }

        if let Some(f) = self.len {
            // Length constraint, same field – FilterDsl interprets this specially
            exprs.push(f.into_scoped(field));
        }

        FilterDsl::all(exprs)
    }
}

///
/// TextFilterKind
///

pub struct TextFilterKind;

impl FilterKind for TextFilterKind {
    type Payload = TextFilter;
}

///
/// TextFilter
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextFilter {
    pub actions: Vec<TextFilterAction>,
    pub is_empty: Option<bool>,
}

impl TextFilter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn is_empty(mut self, val: bool) -> Self {
        self.is_empty = Some(val);
        self
    }

    // --- equality ---
    #[must_use]
    pub fn equal(mut self, value: impl Into<String>) -> Self {
        self.push_action(TextFilterOp::Equal, false, value);
        self
    }

    #[must_use]
    pub fn equal_ci(mut self, value: impl Into<String>) -> Self {
        self.push_action(TextFilterOp::Equal, true, value);
        self
    }

    #[must_use]
    pub fn not_equal(mut self, value: impl Into<String>) -> Self {
        self.push_action(TextFilterOp::NotEqual, false, value);
        self
    }

    #[must_use]
    pub fn not_equal_ci(mut self, value: impl Into<String>) -> Self {
        self.push_action(TextFilterOp::NotEqual, true, value);
        self
    }

    // --- contains ---
    #[must_use]
    pub fn contains(mut self, value: impl Into<String>) -> Self {
        self.push_action(TextFilterOp::Contains, false, value);
        self
    }

    #[must_use]
    pub fn contains_ci(mut self, value: impl Into<String>) -> Self {
        self.push_action(TextFilterOp::Contains, true, value);
        self
    }

    // --- starts_with ---
    #[must_use]
    pub fn starts_with(mut self, value: impl Into<String>) -> Self {
        self.push_action(TextFilterOp::StartsWith, false, value);
        self
    }

    #[must_use]
    pub fn starts_with_ci(mut self, value: impl Into<String>) -> Self {
        self.push_action(TextFilterOp::StartsWith, true, value);
        self
    }

    // --- ends_with ---
    #[must_use]
    pub fn ends_with(mut self, value: impl Into<String>) -> Self {
        self.push_action(TextFilterOp::EndsWith, false, value);
        self
    }

    #[must_use]
    pub fn ends_with_ci(mut self, value: impl Into<String>) -> Self {
        self.push_action(TextFilterOp::EndsWith, true, value);
        self
    }

    // -------- internal ----------
    fn push_action(&mut self, op: TextFilterOp, case_insensitive: bool, value: impl Into<String>) {
        self.actions.push(TextFilterAction {
            op,
            case_insensitive,
            values: vec![value.into()],
        });
    }
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

impl IntoScopedFilterExpr for TextFilter {
    fn into_scoped(self, field: &str) -> FilterExpr {
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
            exprs.push(if is_empty {
                dsl.is_empty(field)
            } else {
                dsl.is_not_empty(field)
            });
        }

        FilterDsl::all(exprs)
    }
}
