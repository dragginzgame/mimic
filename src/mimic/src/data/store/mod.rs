mod data;
mod index;
mod types;

pub use data::*;
pub use index::*;
pub use types::*;

use std::{cell::RefCell, collections::HashMap, rc::Rc, thread::LocalKey};
use thiserror::Error as ThisError;

///
/// StoreError
///

#[derive(Debug, ThisError)]
pub enum StoreError {
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
    pub fn try_get_store(&self, name: &str) -> Result<&'static LocalKey<RefCell<T>>, StoreError> {
        self.0
            .get(name)
            .copied()
            .ok_or_else(|| StoreError::StoreNotFound(name.to_string()))
    }

    // register
    pub fn register(&mut self, name: &'static str, accessor: &'static LocalKey<RefCell<T>>) {
        self.0.insert(name, accessor);
    }

    // with_store
    pub fn with_store<F, R>(&self, name: &str, f: F) -> Result<R, StoreError>
    where
        F: FnOnce(&T) -> R,
    {
        let store = self.try_get_store(name)?;

        Ok(store.with_borrow(|s| f(s)))
    }

    // with_store_mut
    pub fn with_store_mut<F, R>(&self, name: &str, f: F) -> Result<R, StoreError>
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

pub type DataStoreRegistry = &'static LocalKey<Rc<StoreRegistry<DataStore>>>;

pub type IndexStoreRegistry = &'static LocalKey<Rc<StoreRegistry<IndexStore>>>;
