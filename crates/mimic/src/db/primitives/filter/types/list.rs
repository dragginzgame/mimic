use crate::{
    core::traits::FieldValue,
    db::primitives::filter::{
        ContainsFilter, EqualityFilter, FilterDsl, FilterExpr, FilterKind, IntoScopedFilterExpr,
        RangeFilter,
    },
    types::Decimal,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Aliases
///

pub type TextListFilter = ListFilter<String>;
pub type IntListFilter = ListFilter<i64>;
pub type NatListFilter = ListFilter<u64>;
pub type BoolListFilter = ListFilter<bool>;
pub type DecimalListFilter = ListFilter<Decimal>;

///
/// FilterKinds
///

pub struct TextListFilterKind;
impl FilterKind for TextListFilterKind {
    type Payload = TextListFilter;
}

pub struct IntListFilterKind;
impl FilterKind for IntListFilterKind {
    type Payload = IntListFilter;
}

pub struct NatListFilterKind;
impl FilterKind for NatListFilterKind {
    type Payload = NatListFilter;
}

pub struct BoolListFilterKind;
impl FilterKind for BoolListFilterKind {
    type Payload = BoolListFilter;
}

pub struct DecimalListFilterKind;
impl FilterKind for DecimalListFilterKind {
    type Payload = DecimalListFilter;
}

///
/// Generic ListFilter<T>
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ListFilter<T>
where
    T: Default,
{
    /// Membership-based operations on elements (e.g. contains / any_in / all_in)
    pub contains: Option<ContainsFilter<T>>,

    /// Whole-list equality / inequality / in / notIn
    pub eq: Option<EqualityFilter<T>>,

    /// Cardinality constraints (length)
    pub len: Option<RangeFilter<i64>>,
}

impl<T> ListFilter<T>
where
    T: Default,
{
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // ContainsFilter helpers -------------------------------------------------

    #[must_use]
    pub fn contains(mut self, f: impl FnOnce(ContainsFilter<T>) -> ContainsFilter<T>) -> Self {
        let inner = self.contains.take().unwrap_or_default();
        self.contains = Some(f(inner));
        self
    }

    #[must_use]
    pub fn contains_filter(mut self, filter: ContainsFilter<T>) -> Self {
        self.contains = Some(filter);
        self
    }

    // EqualityFilter helpers -------------------------------------------------

    #[must_use]
    pub fn eq(mut self, f: impl FnOnce(EqualityFilter<T>) -> EqualityFilter<T>) -> Self {
        let inner = self.eq.take().unwrap_or_default();
        self.eq = Some(f(inner));
        self
    }

    #[must_use]
    pub fn eq_filter(mut self, filter: EqualityFilter<T>) -> Self {
        self.eq = Some(filter);
        self
    }

    // Length filter helpers --------------------------------------------------

    #[must_use]
    pub fn len(mut self, f: impl FnOnce(RangeFilter<i64>) -> RangeFilter<i64>) -> Self {
        let inner = self.len.take().unwrap_or_default();
        self.len = Some(f(inner));
        self
    }

    #[must_use]
    pub const fn len_filter(mut self, filter: RangeFilter<i64>) -> Self {
        self.len = Some(filter);
        self
    }
}

impl<T> IntoScopedFilterExpr for ListFilter<T>
where
    T: CandidType + Default + FieldValue,
{
    fn into_scoped(self, field: &str) -> FilterExpr {
        let mut exprs = Vec::new();

        if let Some(f) = self.contains {
            exprs.push(f.into_scoped(field));
        }

        if let Some(f) = self.eq {
            exprs.push(f.into_scoped(field));
        }

        if let Some(f) = self.len {
            exprs.push(f.into_scoped(field));
        }

        FilterDsl::all(exprs)
    }
}
