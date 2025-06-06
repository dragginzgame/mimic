mod dynamic;
mod generic;

pub use dynamic::*;
pub use generic::*;

use crate::{
    db::{DataStoreLocal, types::DataRow, types::SortKey},
    storage::{DebugContext, ResolvedSelector},
};

///
/// Loader
///

pub struct Loader {
    store: DataStoreLocal,
    debug: DebugContext,
}

impl Loader {
    // new
    #[must_use]
    pub const fn new(store: DataStoreLocal, debug: DebugContext) -> Self {
        Self { store, debug }
    }

    // load
    pub(crate) fn load(&self, selector: &ResolvedSelector) -> Vec<DataRow> {
        match selector {
            ResolvedSelector::One(key) => {
                self.debug.println(&format!("Loading selector: One({key})"));

                self.query_key(key.clone())
                    .map(|row| vec![row])
                    .unwrap_or_default()
            }

            ResolvedSelector::Many(keys) => {
                self.debug
                    .println(&format!("Loading selector: Many({keys:?})"));

                keys.iter()
                    .filter_map(|key| self.query_key(key.clone()))
                    .collect()
            }

            ResolvedSelector::Range(start, end) => {
                self.debug
                    .println(&format!("Loading selector: Range({start}, {end})"));

                self.query_range(start.clone(), end.clone())
            }
        }
    }

    // query_key
    pub(crate) fn query_key(&self, key: SortKey) -> Option<DataRow> {
        self.store.with_borrow(|this| {
            this.get(&key).map(|value| DataRow {
                key: key.clone(),
                value,
            })
        })
    }

    // query_range
    pub(crate) fn query_range(&self, start: SortKey, end: SortKey) -> Vec<DataRow> {
        self.store.with_borrow(|this| {
            this.range(start..=end)
                .map(|(key, value)| DataRow { key, value })
                .collect()
        })
    }
}
