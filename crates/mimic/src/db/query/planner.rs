use crate::{
    core::{
        Key, Value,
        traits::{EntityKind, IndexKind, IndexKindFn, IndexKindTuple, Path},
    },
    db::{
        query::{Cmp, FilterExpr},
        store::DataKey,
    },
};

///
/// QueryPlan
///

#[derive(Debug)]
pub enum QueryPlan {
    Index(IndexPlan),
    Keys(Vec<DataKey>),
    Range(DataKey, DataKey),
}

///
/// IndexPlan
///

#[derive(Debug)]
pub struct IndexPlan {
    pub store_path: &'static str,
    pub index_path: &'static str,
    pub index_fields: &'static [&'static str],
    pub keys: Vec<Key>,
}

impl IndexPlan {
    #[must_use]
    pub const fn new<I: IndexKind>(keys: Vec<Key>) -> Self {
        Self {
            store_path: I::Store::PATH,
            index_path: I::PATH,
            index_fields: I::FIELDS,
            keys,
        }
    }
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
    pub fn plan<E: EntityKind>(&self) -> QueryPlan {
        // If filter is a primary key match
        // this would handle One and Many queries
        if let Some(plan) = self.extract_from_filter::<E>() {
            return plan;
        }

        // check for index matches
        // THIS WILL DO THE INDEX LOOKUPS
        if E::Indexes::HAS_INDEXES
            && let Some(plan) = self.extract_from_index::<E>()
        {
            return plan;
        }

        // default to the range of the current entity
        let start = DataKey::new::<E>(Key::MIN);
        let end = DataKey::new::<E>(Key::MAX);

        QueryPlan::Range(start, end)
    }

    // extract_from_filter
    fn extract_from_filter<E: EntityKind>(&self) -> Option<QueryPlan> {
        let filter = self.filter.as_ref()?;

        match filter {
            FilterExpr::Clause(clause) if clause.field == E::PRIMARY_KEY => match clause.cmp {
                Cmp::Eq => clause
                    .value
                    .as_key()
                    .map(|key| QueryPlan::Keys(vec![DataKey::new::<E>(key)])),

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
                            Some(QueryPlan::Keys(keys))
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

    // extract_from_index
    fn extract_from_index<E: EntityKind>(&self) -> Option<QueryPlan> {
        let filter = self.filter.as_ref()?;

        let mut matcher = IndexMatcher::new(filter);

        E::Indexes::for_each(&mut matcher).ok()?;

        if let Some(best_match) = matcher.best_match {
            best_match.plan.map(QueryPlan::Index)
        } else {
            None
        }
    }
}

///
/// IndexMatch
///

#[derive(Default)]
struct IndexMatch {
    pub plan: Option<IndexPlan>,
    pub fields_matched: u32,
}

impl IndexMatch {
    // ✅ Consider extracting plan_priority_score (future-proofing)
    pub const fn score(&self) -> u32 {
        self.fields_matched
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
        let fields_matched = keys.len() as u32;

        // set the match
        let new = IndexMatch {
            plan: Some(IndexPlan::new::<I>(keys)),
            fields_matched,
        };

        match &self.best_match {
            Some(existing) if existing.score() >= new.score() => {} // existing is better so skip
            _ => self.best_match = Some(new),
        }

        Ok(())
    }
}
