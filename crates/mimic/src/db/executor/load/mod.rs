mod dynamic;
mod generic;

pub use dynamic::*;
pub use generic::*;

use crate::{
    db::{
        DataError,
        executor::DebugContext,
        store::{DataStoreLocal, DataStoreRegistry, IndexStoreRegistry},
        types::{DataRow, ResolvedSelector, Selector, SortKey, Where},
    },
    def::traits::EntityKind,
};

///
/// Loader
///

pub struct Loader {
    data_reg: DataStoreRegistry,
    index_reg: IndexStoreRegistry,
    debug: DebugContext,
}

impl Loader {
    #[must_use]
    pub const fn new(
        data_reg: DataStoreRegistry,
        index_reg: IndexStoreRegistry,
        debug: DebugContext,
    ) -> Self {
        Self {
            data_reg,
            index_reg,
            debug,
        }
    }

    // load
    pub fn load<E>(
        &self,
        selector: &Selector,
        where_clause: Option<&Where>,
    ) -> Result<Vec<DataRow>, DataError>
    where
        E: EntityKind,
    {
        // does the where clause modify the selector?
        /*
        let selector = match where_clause {
            Some(wc) => self.resolve_selector_with_index(resolved, selector, wc)?,
            None => selector.clone(),
        };
        */

        // get store
        let store = self.data_reg.with(|db| db.try_get_store(E::STORE))?;
        let resolved_selector = selector.resolve::<E>();

        // load rows
        let rows = match resolved_selector {
            ResolvedSelector::One(key) => Self::load_key(store, key).into_iter().collect(),

            ResolvedSelector::Many(keys) => keys
                .into_iter()
                .filter_map(|key| Self::load_key(store, key))
                .collect(),

            ResolvedSelector::Range(start, end) => Self::load_range(store, start, end),
        };

        Ok(rows)
    }

    // load_key
    fn load_key(store: DataStoreLocal, key: SortKey) -> Option<DataRow> {
        store.with_borrow(|this| {
            this.get(&key).map(|value| DataRow {
                key: key.clone(),
                value,
            })
        })
    }

    // load_range
    fn load_range(store: DataStoreLocal, start: SortKey, end: SortKey) -> Vec<DataRow> {
        store.with_borrow(|this| {
            this.range(start..=end)
                .map(|(key, value)| DataRow { key, value })
                .collect()
        })
    }
    /*
    // resolve_selector_with_index
    pub fn resolve_selector_with_index(
        &self,
        resolved: &ResolvedEntity,
        selector: &Selector,
        where_clause: &Where,
    ) -> Result<Selector, DataError> {
        // plan the selector first
        let field_values: HashMap<_, _> = where_clause
            .matches
            .iter()
            .map(|(k, v)| (k.clone(), Some(v.clone())))
            .collect();

        // look for matching indexes
        for index in resolved.indexes() {
            if index.fields.len() == resolved.sk_data.len()
                && index.fields.iter().all(|f| field_values.contains_key(f))
            {
                // no index key, no index lookup
                let Some(index_key) = resolved.build_index_key(index, &field_values) else {
                    continue;
                };

                // load from index store
                let index_store = self.index_reg.with(|map| map.try_get_store(&index.store))?;

                if let Some(index_value) = index_store.with_borrow(|s| s.get(&index_key)) {
                    let keys: Vec<Key> = index_value.iter().cloned().collect();

                    self.debug.println(&format!(
                        "query.load: index hit for {:?} → {} key(s)",
                        index.fields,
                        keys.len()
                    ));

                    let selector = if index.unique {
                        // Just use the first value if the index is guaranteed unique
                        Selector::One(keys.into_iter().next().expect("unique index had no value"))
                    } else {
                        Selector::Many(keys)
                    };

                    return Ok(selector);
                }
            }
        }

        Ok(selector.clone())
    }
    */
}
