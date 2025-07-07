use crate::{
    core::{db::EntityKey, traits::EntityKind},
    db::{
        query::{FilterExpr, ResolvedSelector},
        store::DataKeyRange,
    },
};

///
/// QueryShape
///

#[derive(Debug)]
pub enum QueryShape {
    Single(EntityKey),
    Many(Vec<EntityKey>),
    Range(QueryRange),
    FullScan,
}

///
/// QueryPlan
///

#[derive(Debug)]
pub struct QueryPlan {
    pub shape: QueryShape,
    pub filter: Option<FilterExpr>,
}

impl QueryPlan {
    /// For prefix scans or open-ended range creation
    #[must_use]
    pub fn range_from_prefix(prefix: &EntityKey) -> QueryRange {
        QueryRange {
            start: QueryBound::inclusive_lower_from(prefix),
            end: QueryBound::exclusive_upper_from(prefix),
        }
    }

    #[must_use]
    pub fn from_resolved_selector(selector: ResolvedSelector, filter: Option<FilterExpr>) -> Self {
        let shape = match selector {
            ResolvedSelector::One(key) => QueryShape::Single(key),
            ResolvedSelector::Many(keys) => QueryShape::Many(keys),
            ResolvedSelector::Range(start, end) => QueryShape::Range(QueryRange { start, end }),
        };

        Self { shape, filter }
    }
}

///
/// QueryRange
///

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryRange {
    pub start: QueryBound,
    pub end: QueryBound,
}

impl QueryRange {
    #[must_use]
    pub fn to_data_key_range<E: EntityKind>(&self) -> DataKeyRange {
        let start = E::build_data_key(&self.start.key);
        let end = E::build_data_key(&self.end.key);

        match (self.start.kind, self.end.kind) {
            (BoundKind::Inclusive, BoundKind::Inclusive) => DataKeyRange::Inclusive(start..=end),
            (BoundKind::Inclusive, BoundKind::Exclusive) => DataKeyRange::Exclusive(start..end),
            (BoundKind::Exclusive, BoundKind::Inclusive) => {
                DataKeyRange::SkipFirstInclusive(start..=end)
            }
            (BoundKind::Exclusive, BoundKind::Exclusive) => {
                DataKeyRange::SkipFirstExclusive(start..end)
            }
        }
    }
}

///
/// QueryBound
///

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBound {
    pub key: EntityKey,
    pub kind: BoundKind,
}

impl QueryBound {
    #[must_use]
    pub const fn inclusive(key: EntityKey) -> Self {
        Self {
            key,
            kind: BoundKind::Inclusive,
        }
    }

    #[must_use]
    pub const fn exclusive(key: EntityKey) -> Self {
        Self {
            key,
            kind: BoundKind::Exclusive,
        }
    }

    /// Helper to produce an exclusive lower bound from a prefix key
    #[must_use]
    pub fn inclusive_lower_from(prefix: &EntityKey) -> Self {
        Self {
            key: prefix.clone(),
            kind: BoundKind::Inclusive,
        }
    }

    /// Helper to produce an exclusive upper bound from a prefix key
    #[must_use]
    pub fn exclusive_upper_from(prefix: &EntityKey) -> Self {
        Self {
            key: prefix.clone(),
            kind: BoundKind::Exclusive,
        }
    }
}

///
/// BoundKind
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundKind {
    Inclusive,
    Exclusive,
}
