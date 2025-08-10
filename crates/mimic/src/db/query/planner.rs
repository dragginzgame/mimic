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
    pub values: Vec<Value>,
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

    // extract_from_index: build a leftmost equality prefix in terms of Value
    fn extract_from_index<E: EntityKind>(&self) -> Option<QueryPlan> {
        let Some(filter) = &self.filter else {
            return None;
        };

        let mut best: Option<(usize, IndexPlan)> = None;

        for index in E::INDEXES {
            // Build leftmost equality prefix (only == supported for hashed indexes)
            let mut values: Vec<Value> = Vec::new();

            for field in index.fields {
                if let Some(v) = Self::find_eq_value(filter, field) {
                    values.push(v);
                } else {
                    break; // stop at first non-match
                }
            }

            // Skip indexes that produced no equality prefix
            if values.is_empty() {
                continue;
            }

            let score = values.len();
            let cand = (score, IndexPlan { index, values });

            match &best {
                Some((best_score, _)) if *best_score >= score => { /* keep current best */ }
                _ => best = Some(cand),
            }
        }

        best.map(|(_, plan)| QueryPlan::Index(plan))
    }

    /// Find an equality clause (`field == ?`) anywhere in the filter tree and return the Value.
    fn find_eq_value(filter: &FilterExpr, field: &str) -> Option<Value> {
        match filter {
            FilterExpr::Clause(c) if c.field == field && matches!(c.cmp, Cmp::Eq) => {
                Some(c.value.clone())
            }
            // Walk conjunctive subtrees
            FilterExpr::And(list) => list.iter().find_map(|f| Self::find_eq_value(f, field)),
            _ => None,
        }
    }
}
