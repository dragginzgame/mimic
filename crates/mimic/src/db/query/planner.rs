use crate::{
    core::traits::EntityKind,
    db::{
        query::{Cmp, FilterExpr, RangeExpr},
        store::DataKey,
    },
};

///
/// QueryPlan
///

#[derive(Debug)]
pub struct QueryPlan {
    pub range: Option<RangeExpr>,
    pub filter: Option<FilterExpr>,
}

impl QueryPlan {
    #[must_use]
    pub fn new(range: &Option<RangeExpr>, filter: &Option<FilterExpr>) -> Self {
        Self {
            range: range.clone(),
            filter: filter.clone(),
        }
    }

    #[must_use]
    pub fn shape<E: EntityKind>(&self) -> QueryShape {
        // If a full range is specified
        if let Some(range) = &self.range {
            return QueryShape::Range(range.clone());
        }

        // If filter is a primary key match
        if let Some(shape) = self.extract_primary_key_shape::<E>() {
            return shape;
        }

        // default to the range of the current entity
        QueryShape::Range(RangeExpr::from_entity::<E>())
    }

    fn extract_primary_key_shape<E: EntityKind>(&self) -> Option<QueryShape> {
        let filter = self.filter.as_ref()?;

        match filter {
            FilterExpr::Clause(clause)
                if clause.field == E::PRIMARY_KEY && clause.cmp == Cmp::Eq =>
            {
                if let Some(key) = clause.value.as_key() {
                    let data_key = DataKey::new(E::PATH, key);

                    Some(QueryShape::One(data_key))
                } else {
                    None
                }
            }

            _ => None,
        }
    }
}

///
/// QueryShape
///

pub enum QueryShape {
    All,
    One(DataKey),
    Many(Vec<DataKey>),
    Range(RangeExpr),
}
