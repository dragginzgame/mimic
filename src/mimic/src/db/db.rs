use super::store::Store;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{cell::RefCell, collections::HashMap, thread::LocalKey};

///
/// DbError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum DbError {
    #[snafu(display("store not found: {path}"))]
    StoreNotFound { path: String },
}

impl DbError {
    #[must_use]
    pub fn store_not_found(path: &str) -> Self {
        Self::StoreNotFound {
            path: path.to_string(),
        }
    }
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
        F: FnOnce(&Store) -> Result<R, DbError>,
    {
        let res = self
            .stores
            .get(path)
            .ok_or_else(|| DbError::store_not_found(path))
            .and_then(|local_key| local_key.with(|store| f(&store.borrow())))?;

        Ok(res)
    }

    // with_store_mut
    pub fn with_store_mut<F, R>(&self, path: &str, f: F) -> Result<R, DbError>
    where
        F: FnOnce(&mut Store) -> Result<R, DbError>,
    {
        let res = self
            .stores
            .get(path)
            .ok_or_else(|| DbError::store_not_found(path))
            .and_then(|local_key| local_key.with(|store| f(&mut store.borrow_mut())))?;

        Ok(res)
    }
}
