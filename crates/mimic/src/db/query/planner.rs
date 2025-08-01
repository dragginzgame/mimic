use crate::{
    core::{Key, Value, traits::EntityKind},
    db::{
        query::{Cmp, FilterExpr},
        store::DataKey,
    },
    schema::node::Index,
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
    pub index: &'static Index,
    pub keys: Vec<Key>,
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
        if !E::INDEXES.is_empty()
            && let Some(plan) = self.extract_from_index::<E>()
        {
            return plan;
        }

        // Fallback: if we have a real filter, do a full scan
        // No filter = full scan from Key::MIN to Key::MAX
        let min = DataKey::new::<E>(Key::MIN);
        let max = DataKey::new::<E>(Key::MAX);

        QueryPlan::Range(min, max)
    }

    // extract_from_filter
    fn extract_from_filter<E: EntityKind>(&self) -> Option<QueryPlan> {
        let Some(filter) = &self.filter else {
            return None;
        };

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
        let Some(filter) = &self.filter else {
            return None;
        };

        let mut best: Option<IndexMatch> = None;

        for index in E::INDEXES {
            let mut keys = Vec::new();

            for field in index.fields {
                match Self::find_eq_clause(filter, field) {
                    Some(k) => keys.push(k),
                    None => break, // stop at first non-match
                }
            }

            // get score
            let score = keys.len() as u32;
            let plan = IndexPlan { index, keys };

            let new_match = IndexMatch {
                plan: Some(plan),
                fields_matched: score,
            };

            match &best {
                Some(existing) if existing.score() >= new_match.score() => {}
                _ => best = Some(new_match),
            }
        }

        best.and_then(|m| m.plan).map(QueryPlan::Index)
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
