use crate::{
    core::{
        Key, Value,
        traits::{EntityKind, IndexKind, IndexKindFn, IndexKindTuple, Path},
    },
    db::{
        query::{Cmp, FilterExpr},
        store::{DataKey, IndexId, IndexStoreRegistryLocal},
    },
};

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

///
/// QueryPlanner
///

#[derive(Debug)]
pub struct QueryPlanner {
    pub filter: Option<FilterExpr>,
    pub index_registry: IndexStoreRegistryLocal,
}

impl QueryPlanner {
    #[must_use]
    pub fn new(filter: &Option<FilterExpr>, index_registry: IndexStoreRegistryLocal) -> Self {
        Self {
            filter: filter.clone(),
            index_registry,
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
        // THIS WILL DO THE INDEX LOOKUPS
        if let Some(shape) = self.extract_index_shape::<E>() {
            return shape;
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

        let mut matcher = IndexMatcher::new(filter);

        E::Indexes::for_each(&mut matcher).ok()?;

        matcher.best_match.map(|matched| {
            let index_id = IndexId::new(&matched.index_path, matched.fields);
            let index_store = self
                .index_registry
                .with(|reg| reg.get_store_by_path(&matched.store_path));

            let keys: Vec<Key> = index_store.with_borrow(|store| {
                store
                    .range_with_prefix(&index_id, &matched.keys)
                    .flat_map(|(_, entry)| entry.iter().copied().collect::<Vec<_>>())
                    .collect()
            });

            match keys.len() {
                0 => None,
                1 => Some(QueryShape::One(DataKey::new::<E>(
                    keys.into_iter().next().unwrap(),
                ))),
                _ => Some(QueryShape::Many(
                    keys.into_iter().map(DataKey::new::<E>).collect(),
                )),
            }
        })?
    }
}

///
/// IndexMatch
///

#[derive(Default)]
struct IndexMatch {
    pub store_path: String,
    pub index_path: String,
    pub fields: &'static [&'static str],
    pub keys: Vec<Key>,
    pub fields_matched: usize,
}

///
/// IndexMatcher
///

struct IndexMatcher {
    pub filter: FilterExpr,
    pub best_match: Option<IndexMatch>,
}

impl IndexMatcher {
    fn new(filter: &FilterExpr) -> Self {
        Self {
            filter: filter.clone(),
            best_match: None,
        }
    }

    fn find_eq_clause(filter: &FilterExpr, field: &str) -> Option<Key> {
        match filter {
            FilterExpr::Clause(c) if c.field == field && matches!(c.cmp, Cmp::Eq) => {
                Some(c.value.as_key()?)
            }
            FilterExpr::And(list) => list.iter().find_map(|f| Self::find_eq_clause(f, field)),
            _ => None,
        }
    }
}

impl IndexKindFn for IndexMatcher {
    type Error = ();

    fn apply<I: IndexKind>(&mut self) -> Result<(), Self::Error> {
        // Match all fields in the index
        let mut keys = Vec::new();
        for &field in I::FIELDS {
            match Self::find_eq_clause(&self.filter, field) {
                Some(k) => keys.push(k),
                None => break, // stop at first non-match
            }
        }

        let fields_matched = keys.len();
        let new = IndexMatch {
            store_path: I::Store::path(),
            index_path: I::path(),
            fields: I::FIELDS,
            keys,
            fields_matched,
        };

        match &self.best_match {
            Some(existing) if existing.fields_matched >= fields_matched => {} // existing is better
            _ => self.best_match = Some(new),
        }

        Ok(())
    }
}
