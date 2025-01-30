use super::types::{DataKey, DataValue};
use crate::ic::structures::{btreemap::BTreeMap, DefaultMemory};
use derive_more::{Deref, DerefMut};
use std::{cell::RefCell, thread::LocalKey};

///
/// Store
///

#[derive(Deref, DerefMut)]
pub struct Store {
    pub data: BTreeMap<DataKey, DataValue>,
}

impl Store {
    // init
    #[must_use]
    pub fn init(memory: DefaultMemory) -> Self {
        Self {
            data: BTreeMap::init(memory),
        }
    }
}

pub type StoreLocal = &'static LocalKey<RefCell<Store>>;
