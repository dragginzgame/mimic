pub mod cache;
pub mod types;

pub use types::{DataKey, DataRow, DataValue, Metadata};

use derive_more::{Deref, DerefMut};
use ic::structures::{memory::VirtualMemory, BTreeMap};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{cell::RefCell, collections::HashMap, thread::LocalKey};

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("store not found: {path}"))]
    StoreNotFound { path: String },
}

impl Error {
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
    pub fn with_store<F, R>(&self, name: &str, f: F) -> Result<R, Error>
    where
        F: FnOnce(&Store) -> Result<R, Error>,
    {
        self.stores
            .get(name)
            .ok_or_else(|| Error::store_not_found(name))
            .and_then(|local_key| local_key.with(|store| f(&store.borrow())))
    }

    // with_store_mut
    pub fn with_store_mut<F, R>(&self, name: &str, f: F) -> Result<R, Error>
    where
        F: FnOnce(&mut Store) -> Result<R, Error>,
    {
        self.stores
            .get(name)
            .ok_or_else(|| Error::store_not_found(name))
            .and_then(|local_key| local_key.with(|store| f(&mut store.borrow_mut())))
    }
}

///
/// Store
/// a wrapper around the stable BTreeMap with a reference to Schema
/// used to generate QueryBuilders to keep the code modular
///

#[derive(Deref, DerefMut)]
pub struct Store {
    pub data: BTreeMap<DataKey, DataValue>,
}

impl Store {
    // init
    #[must_use]
    pub fn init(memory: VirtualMemory) -> Self {
        Self {
            data: BTreeMap::init(memory),
        }
    }
}
