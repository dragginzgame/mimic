use crate::{
    core::{Key, Value, traits::EntityKind},
    db::{
        query::{Cmp, FilterExpr},
        store::DataKey,
    },
};

///
/// QueryPlan
///

#[derive(Debug)]
pub struct QueryPlan {
    pub filter: Option<FilterExpr>,
}

impl QueryPlan {
    #[must_use]
    pub fn new(filter: &Option<FilterExpr>) -> Self {
        Self {
            filter: filter.clone(),
        }
    }

    #[must_use]
    pub fn shape<E: EntityKind>(&self) -> QueryShape {
        // If filter is a primary key match
        // this would handle One and Many queries
        if let Some(shape) = self.extract_primary_key_shape::<E>() {
            return shape;
        }

        // default to the range of the current entity
        let start = DataKey::new(E::PATH, Key::MIN);
        let end = DataKey::new(E::PATH, Key::MAX);

        QueryShape::Range(start, end)
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

                        if keys.is_empty() {
                            None
                        } else {
                            Some(QueryShape::Many(keys))
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
    Range(DataKey, DataKey),
}
