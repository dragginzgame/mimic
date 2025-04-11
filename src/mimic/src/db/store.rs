use crate::{
    db::types::DataValue,
    ic::structures::{DefaultMemory, btreemap::BTreeMap},
    orm::base::types::SortKey,
};
use derive_more::{Deref, DerefMut};
use std::{cell::RefCell, thread::LocalKey};

///
/// Store
///

#[derive(Deref, DerefMut)]
pub struct Store {
    pub data: BTreeMap<SortKey, DataValue>,
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

///
/// StoreLocal
///

pub type StoreLocal = &'static LocalKey<RefCell<Store>>;
