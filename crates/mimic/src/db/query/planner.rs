use crate::{
    core::traits::EntityKind,
    db::{
        query::FilterExpr,
        store::{DataKey, DataKeyRange},
    },
};

///
/// QueryPlan
///

#[derive(Debug)]
pub struct QueryPlan {
    pub shape: QueryShape,
    pub filter: Option<FilterExpr>,
}

impl QueryPlan {
    #[must_use]
    pub const fn new(shape: QueryShape, filter: Option<FilterExpr>) -> Self {
        Self { shape, filter }
    }
}

///
/// QueryShape
///

#[derive(Debug)]
pub enum QueryShape {
    All,
    One(DataKey),
    Many(Vec<DataKey>),
    Range(QueryRange),
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
    pub const fn new(start: QueryBound, end: QueryBound) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn to_data_key_range<E: EntityKind>(self) -> DataKeyRange {
        let start = self.start.key;
        let end = self.end.key;

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
    pub key: DataKey,
    pub kind: BoundKind,
}

impl QueryBound {
    #[must_use]
    pub const fn inclusive(key: DataKey) -> Self {
        Self {
            key,
            kind: BoundKind::Inclusive,
        }
    }

    #[must_use]
    pub const fn exclusive(key: DataKey) -> Self {
        Self {
            key,
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
