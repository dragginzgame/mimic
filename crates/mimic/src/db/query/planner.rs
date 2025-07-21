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
/// QueryPlan
///

#[derive(Debug)]
pub struct QueryPlan {
    pub shape: QueryShape,
    pub index_used: Option<IndexId>,
    pub matched_keys: usize,
}

impl QueryPlan {
    #[must_use]
    pub const fn from_shape(shape: QueryShape) -> Self {
        Self {
            shape,
            index_used: None,
            matched_keys: 0,
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

///
/// QueryPlanner
///

#[derive(Debug)]
pub struct QueryPlanner {
    pub filter: Option<FilterExpr>,
}

impl QueryPlanner {
    #[must_use]
    pub fn new(filter: Option<&FilterExpr>) -> Self {
        Self {
            filter: filter.cloned(),
        }
    }

    #[must_use]
    pub fn plan_with_registry<E: EntityKind>(
        &self,
        registry: IndexStoreRegistryLocal,
    ) -> QueryPlan {
        // If filter is a primary key match
        // this would handle One and Many queries
        if let Some(shape) = self.extract_shape::<E>() {
            return QueryPlan::from_shape(shape);
        }

        // check for index matches
        // THIS WILL DO THE INDEX LOOKUPS
        if E::Indexes::HAS_INDEXES
            && let Some(plan) = self.extract_index_plan::<E>(registry)
        {
            return plan;
        }

        // default to the range of the current entity
        let start = DataKey::new::<E>(Key::MIN);
        let end = DataKey::new::<E>(Key::MAX);

        QueryPlan::from_shape(QueryShape::Range(start, end))
    }

    // extract_shape
    // currently using primary key lookups
    fn extract_shape<E: EntityKind>(&self) -> Option<QueryShape> {
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

    // extract_index_plan
    fn extract_index_plan<E: EntityKind>(
        &self,
        registry: IndexStoreRegistryLocal,
    ) -> Option<QueryPlan> {
        let filter = self.filter.as_ref()?;

        let mut matcher = IndexMatcher::new(filter);

        E::Indexes::for_each(&mut matcher).ok()?;

        matcher.best_match.map(|matched| {
            let index_id = IndexId::new(&matched.index_path, matched.fields);
            let index_store = registry.with(|reg| reg.get_store_by_path(&matched.store_path));

            let keys: Vec<Key> = index_store.with_borrow(|store| {
                store
                    .range_with_prefix(&index_id, &matched.keys)
                    .flat_map(|(_, entry)| entry.iter().copied().collect::<Vec<_>>())
                    .collect()
            });

            let matched_keys = keys.len();
            let shape = match &matched_keys {
                0 => return None,
                1 => QueryShape::One(DataKey::new::<E>(keys.into_iter().next().unwrap())),
                _ => QueryShape::Many(keys.into_iter().map(DataKey::new::<E>).collect()),
            };

            Some(QueryPlan {
                shape,
                index_used: Some(index_id),
                matched_keys,
            })
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

impl IndexMatch {
    fn new<I: IndexKind>(keys: Vec<Key>) -> Self {
        Self {
            store_path: I::Store::path(),
            index_path: I::path(),
            fields: I::FIELDS,
            fields_matched: keys.len(),
            keys,
        }
    }
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

        // set the match
        let fields_matched = keys.len();
        let new = IndexMatch::new::<I>(keys);

        match &self.best_match {
            Some(existing) if existing.fields_matched >= fields_matched => {} // existing is better
            _ => self.best_match = Some(new),
        }

        Ok(())
    }
}
