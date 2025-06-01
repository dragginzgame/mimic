pub mod store;
pub mod types;

pub use store::*;

use std::{cell::RefCell, collections::HashMap, rc::Rc, thread::LocalKey};
use thiserror::Error as ThisError;

///
/// DbError
///

#[derive(Debug, ThisError)]
pub enum DbError {
    #[error("store not found: {0}")]
    StoreNotFound(String),
}

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

    // try_get_store
    pub fn try_get_store(&self, name: &str) -> Result<&'static LocalKey<RefCell<T>>, DbError> {
        self.0
            .get(name)
            .copied()
            .ok_or_else(|| DbError::StoreNotFound(name.to_string()))
    }

    // register
    pub fn register(&mut self, name: &'static str, accessor: &'static LocalKey<RefCell<T>>) {
        self.0.insert(name, accessor);
    }

    // with_store
    pub fn with_store<F, R>(&self, name: &str, f: F) -> Result<R, DbError>
    where
        F: FnOnce(&T) -> R,
    {
        let store = self.try_get_store(name)?;

        Ok(store.with_borrow(|s| f(s)))
    }

    // with_store_mut
    pub fn with_store_mut<F, R>(&self, name: &str, f: F) -> Result<R, DbError>
    where
        F: FnOnce(&mut T) -> R,
    {
        let store = self.try_get_store(name)?;

        Ok(store.with_borrow_mut(|s| f(s)))
    }
}

///
/// Local Variables
///

pub type DataStoreRegistryLocal = &'static LocalKey<Rc<StoreRegistry<DataStore>>>;

pub type IndexStoreRegistryLocal = &'static LocalKey<Rc<StoreRegistry<IndexStore>>>;
