mod data;
mod index;

pub use data::*;
pub use index::*;

use std::{cell::RefCell, collections::HashMap, rc::Rc, thread::LocalKey};
use thiserror::Error as ThisError;

///
/// StoreError
///

#[derive(Debug, ThisError)]
pub enum StoreError {
    #[error("store '{0}' not found")]
    StoreNotFound(String),
}

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

    // register
    pub fn register(&mut self, name: &'static str, accessor: &'static LocalKey<RefCell<T>>) {
        self.0.insert(name, accessor);
    }

    // try_get_store
    pub fn try_get_store(&self, path: &str) -> Result<&'static LocalKey<RefCell<T>>, StoreError> {
        self.0
            .get(path)
            .copied()
            .ok_or_else(|| StoreError::StoreNotFound(path.to_string()))
    }

    // with_store
    pub fn with_store<R>(&self, path: &str, f: impl FnOnce(&T) -> R) -> Result<R, StoreError> {
        let store = self.try_get_store(path)?;

        Ok(store.with_borrow(|s| f(s)))
    }

    // with_store_mut
    pub fn with_store_mut<R>(
        &self,
        path: &str,
        f: impl FnOnce(&mut T) -> R,
    ) -> Result<R, StoreError> {
        let store = self.try_get_store(path)?;

        Ok(store.with_borrow_mut(|s| f(s)))
    }
}
