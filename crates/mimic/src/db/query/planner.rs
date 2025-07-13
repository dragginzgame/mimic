use crate::{
    core::{Value, traits::EntityKind},
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
        // this would handle One and Many queries
        if let Some(shape) = self.extract_primary_key_shape::<E>() {
            return shape;
        }

        // default to the range of the current entity
        QueryShape::Range(RangeExpr::from_entity::<E>())
    }

    // extract_primary_key_shape
    fn extract_primary_key_shape<E: EntityKind>(&self) -> Option<QueryShape> {
        let filter = self.filter.as_ref()?;

        match filter {
            FilterExpr::Clause(clause) if clause.field == E::PRIMARY_KEY => match clause.cmp {
                Cmp::Eq => clause
                    .value
                    .as_key()
                    .map(|key| QueryShape::One(DataKey::new(E::PATH, key))),

                Cmp::In => {
                    if let Value::List(values) = &clause.value {
                        let keys = values
                            .iter()
                            .filter_map(|v| v.as_ref().as_key())
                            .map(|key| DataKey::new(E::PATH, key))
                            .collect::<Vec<_>>();

                        if !keys.is_empty() {
                            Some(QueryShape::Many(keys))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }

                _ => None,
            },

            _ => None,
        }
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
    Range(RangeExpr),
}
