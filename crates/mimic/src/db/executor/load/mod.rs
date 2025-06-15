mod dynamic;
mod generic;

pub use dynamic::*;
pub use generic::*;

use crate::{
    db::{
        DataError,
        store::{DataStoreLocal, DataStoreRegistry, IndexStoreRegistry},
        types::{DataRow, ResolvedSelector, Selector, SortKey, Where},
    },
    def::traits::EntityKind,
};

///
/// Loader
///

pub struct Loader {
    data_registry: DataStoreRegistry,
    index_registry: IndexStoreRegistry,
    debug: bool,
}

impl Loader {
    #[must_use]
    pub const fn new(
        data_registry: DataStoreRegistry,
        index_registry: IndexStoreRegistry,
        debug: bool,
    ) -> Self {
        Self {
            data_registry,
            index_registry,
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
        // TODO - big where_clause changing selector thingy
        // get store
        let store = self.data_registry.with(|db| db.try_get_store(E::STORE))?;
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
        ::icu::ic::println!("load_key : {key}");

        store.with_borrow(|this| {
            this.get(&key).map(|value| DataRow {
                key: key.clone(),
                value,
            })
        })
    }

    // load_range
    fn load_range(store: DataStoreLocal, start: SortKey, end: SortKey) -> Vec<DataRow> {
        ::icu::ic::println!("load_range : {start}, {end}");

        store.with_borrow(|this| {
            this.range(start..=end)
                .map(|(key, value)| DataRow { key, value })
                .collect()
        })
    }
}
