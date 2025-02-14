pub mod store;
pub mod types;

pub use store::{Store, StoreLocal};

use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, thread::LocalKey};
use thiserror::Error as ThisError;

///
/// DbError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum DbError {
    #[error("store not found: {0}")]
    StoreNotFound(String),
}

///
/// Db
///

#[derive(Default)]
pub struct Db {
    stores: HashMap<&'static str, &'static LocalKey<RefCell<Store>>>,
}

impl Db {
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // insert
    pub fn insert(&mut self, name: &'static str, accessor: &'static LocalKey<RefCell<Store>>) {
        self.stores.insert(name, accessor);
    }

    // with_store
    pub fn with_store<F, R>(&self, path: &str, f: F) -> Result<R, DbError>
    where
        F: FnOnce(&Store) -> R,
    {
        let store = self
            .stores
            .get(path)
            .ok_or_else(|| DbError::StoreNotFound(path.to_string()))?;

        Ok(store.with_borrow(|store| f(store)))
    }

    // with_store_mut
    pub fn with_store_mut<F, R>(&self, path: &str, f: F) -> Result<R, DbError>
    where
        F: FnOnce(&mut Store) -> R,
    {
        let store = self
            .stores
            .get(path)
            .ok_or_else(|| DbError::StoreNotFound(path.to_string()))?;

        Ok(store.with_borrow_mut(|store| f(store)))
    }
}
