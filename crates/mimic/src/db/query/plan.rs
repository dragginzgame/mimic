use crate::{
    core::{Key, Value, traits::EntityKind},
    db::{
        query::{Cmp, FilterExpr},
        store::DataKey,
    },
    schema::node::EntityIndex,
};

///
/// QueryPlan
///

#[derive(Debug, Default)]
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
        if let Some(shape) = self.extract_pk_shape::<E>() {
            return shape;
        }

        // check for index matches
        if let Some(index_shape) = self.extract_index_shape::<E>() {
            return index_shape;
        }

        // default to the range of the current entity
        let start = DataKey::new::<E>(Key::MIN);
        let end = DataKey::new::<E>(Key::MAX);

        QueryShape::Range(start, end)
    }

    // extract_pk_shape
    fn extract_pk_shape<E: EntityKind>(&self) -> Option<QueryShape> {
        let filter = self.filter.as_ref()?;

        match filter {
            FilterExpr::Clause(clause) if clause.field == E::PRIMARY_KEY => match clause.cmp {
                Cmp::Eq => clause
                    .value
                    .as_key()
                    .map(|key| QueryShape::One(DataKey::new::<E>(key))),

                Cmp::In => {
                    if let Value::List(values) = &clause.value {
                        let keys = values
                            .iter()
                            .filter_map(|v| v.as_ref().as_key())
                            .map(|key| DataKey::new::<E>(key))
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

    fn extract_index_shape<E: EntityKind>(&self) -> Option<QueryShape> {
        let filter = self.filter.as_ref()?;
        let mut best_match: Option<(EntityIndex, Vec<Key>)> = None;

        for index in E::INDEXES {
            let mut matched_keys = vec![];

            for &field in index.fields {
                match Self::find_eq_clause(filter, field) {
                    Some(v) => match v.as_key() {
                        Some(k) => matched_keys.push(k),
                        None => break,
                    },
                    None => break,
                }
            }

            if matched_keys.is_empty() {
                continue; // skip this index entirely
            }

            if let Some((_, best_keys)) = &best_match {
                if matched_keys.len() > best_keys.len() {
                    best_match = Some((index.clone(), matched_keys));
                }
            } else {
                best_match = Some((index.clone(), matched_keys));
            }
        }

        best_match.map(|(index, keys)| QueryShape::Index { index, keys })
    }

    fn find_eq_clause<'a>(filter: &'a FilterExpr, field: &str) -> Option<&'a Value> {
        match filter {
            FilterExpr::Clause(c) if c.field == field && matches!(c.cmp, Cmp::Eq) => Some(&c.value),
            FilterExpr::And(list) => list.iter().find_map(|f| Self::find_eq_clause(f, field)),
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
    Index { index: EntityIndex, keys: Vec<Key> },
}
