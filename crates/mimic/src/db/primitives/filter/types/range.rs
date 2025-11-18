use crate::{
    core::traits::FieldValue,
    db::primitives::filter::{FilterDsl, FilterExpr, FilterKind, IntoScopedFilterExpr},
    types::Decimal,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// RangeDecimalFilterKind
///

pub struct RangeDecimalFilterKind;

impl FilterKind for RangeDecimalFilterKind {
    type Payload = RangeFilter<Decimal>;
}

///
/// RangeIntFilterKind
///

pub struct RangeIntFilterKind;

impl FilterKind for RangeIntFilterKind {
    type Payload = RangeFilter<i64>;
}

///
/// RangeNatFilterKind
///

pub struct RangeNatFilterKind;

impl FilterKind for RangeNatFilterKind {
    type Payload = RangeFilter<u64>;
}

///
/// RangeFilter
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct RangeFilter<V> {
    pub gt: Option<V>,
    pub gte: Option<V>,
    pub lt: Option<V>,
    pub lte: Option<V>,
    pub between: Option<(V, V)>,
}

impl<V> RangeFilter<V>
where
    V: CandidType + Default + FieldValue,
{
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn gt(mut self, v: impl Into<V>) -> Self {
        self.gt = Some(v.into());
        self
    }

    #[must_use]
    pub fn gte(mut self, v: impl Into<V>) -> Self {
        self.gte = Some(v.into());
        self
    }

    #[must_use]
    pub fn lt(mut self, v: impl Into<V>) -> Self {
        self.lt = Some(v.into());
        self
    }

    #[must_use]
    pub fn lte(mut self, v: impl Into<V>) -> Self {
        self.lte = Some(v.into());
        self
    }

    #[must_use]
    pub fn between(mut self, min: impl Into<V>, max: impl Into<V>) -> Self {
        self.between = Some((min.into(), max.into()));
        self
    }
}

impl<V> IntoScopedFilterExpr for RangeFilter<V>
where
    V: CandidType + Default + FieldValue,
{
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
