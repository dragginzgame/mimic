use crate::{
    core::{Value, traits::FieldValue},
    db::primitives::filter::{FilterDsl, FilterExpr, FilterKind, IntoScopedFilterExpr},
    types::Decimal,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Aliases
///

pub type TextContainsFilter = ContainsFilter<String>;
pub type IntContainsFilter = ContainsFilter<i64>;
pub type NatContainsFilter = ContainsFilter<u64>;
pub type BoolContainsFilter = ContainsFilter<bool>;
pub type DecimalContainsFilter = ContainsFilter<Decimal>;
pub type ValueContainsFilter = ContainsFilter<Value>;

///
/// FilterKinds
///

pub struct TextContainsFilterKind;
impl FilterKind for TextContainsFilterKind {
    type Payload = TextContainsFilter;
}

pub struct IntContainsFilterKind;
impl FilterKind for IntContainsFilterKind {
    type Payload = IntContainsFilter;
}

pub struct NatContainsFilterKind;
impl FilterKind for NatContainsFilterKind {
    type Payload = NatContainsFilter;
}

pub struct BoolContainsFilterKind;
impl FilterKind for BoolContainsFilterKind {
    type Payload = BoolContainsFilter;
}

pub struct DecimalContainsFilterKind;
impl FilterKind for DecimalContainsFilterKind {
    type Payload = DecimalContainsFilter;
}

///
/// ContainsFilter
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ContainsFilter<T>
where
    T: Default,
{
    pub contains: Option<T>,
    pub all_in: Option<Vec<T>>,
    pub any_in: Option<Vec<T>>,

    pub not_contains: Option<T>,
    pub not_any_in: Option<Vec<T>>,
    pub not_all_in: Option<Vec<T>>,

    pub is_empty: Option<bool>,
}

impl<T> ContainsFilter<T>
where
    T: Default,
{
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // ---- each builder method now takes T, not Value ----

    #[must_use]
    pub fn contains(mut self, value: T) -> Self {
        self.contains = Some(value);
        self
    }

    #[must_use]
    pub fn not_contains(mut self, value: T) -> Self {
        self.not_contains = Some(value);
        self
    }

    #[must_use]
    pub fn any_in<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self::extend_slot(&mut self.any_in, values);
        self
    }

    #[must_use]
    pub fn all_in<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self::extend_slot(&mut self.all_in, values);
        self
    }

    #[must_use]
    pub fn not_any_in<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self::extend_slot(&mut self.not_any_in, values);
        self
    }

    #[must_use]
    pub fn not_all_in<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
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

    fn extend_slot<I>(slot: &mut Option<Vec<T>>, values: I)
    where
        I: IntoIterator<Item = T>,
    {
        slot.get_or_insert_with(Vec::new).extend(values);
    }
}

impl<T> IntoScopedFilterExpr for ContainsFilter<T>
where
    T: CandidType + Default + FieldValue,
{
    fn into_scoped(self, path: &str) -> FilterExpr {
        let dsl = FilterDsl;
        let mut exprs = vec![];

        // positive variants
        if let Some(v) = self.contains {
            exprs.push(dsl.contains(path, v.to_value()));
        }
        if let Some(vs) = self.any_in {
            let vs: Vec<_> = vs.into_iter().map(|v| v.to_value()).collect();
            exprs.push(dsl.any_in(path, vs));
        }
        if let Some(vs) = self.all_in {
            let vs: Vec<_> = vs.into_iter().map(|v| v.to_value()).collect();
            exprs.push(dsl.all_in(path, vs));
        }

        // negative variants
        if let Some(v) = self.not_contains {
            exprs.push(FilterExpr::Not(Box::new(dsl.contains(path, v.to_value()))));
        }
        if let Some(vs) = self.not_any_in {
            let vs: Vec<_> = vs.into_iter().map(|v| v.to_value()).collect();
            exprs.push(FilterExpr::Not(Box::new(dsl.any_in(path, vs))));
        }
        if let Some(vs) = self.not_all_in {
            let vs: Vec<_> = vs.into_iter().map(|v| v.to_value()).collect();
            exprs.push(FilterExpr::Not(Box::new(dsl.all_in(path, vs))));
        }

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
