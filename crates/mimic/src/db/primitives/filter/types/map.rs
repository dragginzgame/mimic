use crate::{
    core::Value,
    db::primitives::filter::{FilterDsl, FilterExpr, IntoScopedFilterExpr, RangeFilter},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// MapFilter
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct MapFilter {
    pub contains_key: Option<Value>,
    pub not_contains_key: Option<Value>,
    pub contains_value: Option<Value>,
    pub not_contains_value: Option<Value>,
    pub contains_entry: Option<(Value, Value)>,
    pub not_contains_entry: Option<(Value, Value)>,
    pub len: Option<RangeFilter<i64>>,
}

impl IntoScopedFilterExpr for MapFilter {
    fn into_scoped(self, field: &str) -> FilterExpr {
        let dsl = FilterDsl;
        let mut exprs = Vec::new();

        if let Some(k) = self.contains_key {
            exprs.push(dsl.map_contains_key(field, k));
        }
        if let Some(k) = self.not_contains_key {
            exprs.push(FilterExpr::Not(Box::new(dsl.map_contains_key(field, k))));
        }

        if let Some(v) = self.contains_value {
            exprs.push(dsl.map_contains_value(field, v));
        }
        if let Some(v) = self.not_contains_value {
            exprs.push(FilterExpr::Not(Box::new(dsl.map_contains_value(field, v))));
        }

        if let Some((k, v)) = self.contains_entry {
            exprs.push(dsl.map_contains_entry(field, k, v));
        }
        if let Some((k, v)) = self.not_contains_entry {
            exprs.push(FilterExpr::Not(Box::new(
                dsl.map_contains_entry(field, k, v),
            )));
        }

        if let Some(r) = self.len {
            exprs.push(r.into_scoped(field));
        }

        FilterDsl::all(exprs)
    }
}
