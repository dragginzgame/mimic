mod data;
mod index;

pub use data::*;
pub use index::*;

use crate::core::traits::StoreKind;
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

    // get_store_by_path
    #[must_use]
    pub fn get_store_by_path(&self, path: &str) -> &'static LocalKey<RefCell<T>> {
        self.0
            .get(path)
            .copied()
            .unwrap_or_else(|| panic!("store '{path}' not found"))
    }

    // get_store
    #[must_use]
    pub fn get_store<S: StoreKind>(&self) -> &'static LocalKey<RefCell<T>> {
        self.0
            .get(S::PATH)
            .copied()
            .unwrap_or_else(|| panic!("store '{}' not found", S::PATH))
    }

    // register
    pub fn register(&mut self, name: &'static str, accessor: &'static LocalKey<RefCell<T>>) {
        self.0.insert(name, accessor);
    }

    // with_store
    pub fn with_store<S, R>(&self, f: impl FnOnce(&T) -> R) -> R
    where
        S: StoreKind,
    {
        let store = self.get_store::<S>();

        store.with_borrow(|s| f(s))
    }

    // with_store_mut
    pub fn with_store_mut<S, R>(&self, f: impl FnOnce(&mut T) -> R) -> R
    where
        S: StoreKind,
    {
        let store = self.get_store::<S>();

        store.with_borrow_mut(|s| f(s))
    }
}
