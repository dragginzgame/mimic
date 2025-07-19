mod data;
mod index;

pub use data::*;
pub use index::*;

use crate::core::traits::{HasStore, Path};
use std::{cell::RefCell, collections::HashMap, rc::Rc, thread::LocalKey};

///
/// StoreRegistryLocal
///

pub type DataStoreRegistryLocal = &'static LocalKey<Rc<StoreRegistry<DataStore>>>;

pub type IndexStoreRegistryLocal = &'static LocalKey<Rc<StoreRegistry<IndexStore>>>;

///
/// StoreRegistry
///

#[derive(Default)]
pub struct StoreRegistry<T: 'static>(HashMap<&'static str, &'static LocalKey<RefCell<T>>>);

impl<T: 'static> StoreRegistry<T> {
    // new
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // get_store
    #[must_use]
    pub fn get_store<H: HasStore>(&self) -> &'static LocalKey<RefCell<T>> {
        self.0
            .get(H::Store::PATH)
            .copied()
            .unwrap_or_else(|| panic!("store '{}' not found", H::Store::PATH))
    }

    // register
    pub fn register(&mut self, name: &'static str, accessor: &'static LocalKey<RefCell<T>>) {
        self.0.insert(name, accessor);
    }

    // with_store
    pub fn with_store<H, F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
        H: HasStore,
    {
        let store = self.get_store::<H>();

        store.with_borrow(|s| f(s))
    }

    // with_store_mut
    pub fn with_store_mut<H, F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
        H: HasStore,
    {
        let store = self.get_store::<H>();

        store.with_borrow_mut(|s| f(s))
    }
}
