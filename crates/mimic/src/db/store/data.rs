use crate::{
    db::types::{DataValue, SortKey},
    ic::structures::{BTreeMap, DefaultMemory},
};
use derive_more::{Deref, DerefMut};
use std::{cell::RefCell, thread::LocalKey};

///
/// DataStore
///

#[derive(Deref, DerefMut)]
pub struct DataStore(BTreeMap<SortKey, DataValue>);

impl DataStore {
    #[must_use]
    pub fn init(memory: DefaultMemory) -> Self {
        Self(BTreeMap::init(memory))
    }
}

///
/// DataStoreLocal
///

pub type DataStoreLocal = &'static LocalKey<RefCell<DataStore>>;
