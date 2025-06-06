use crate::{
    data::store::types::IndexKey,
    ic::structures::{BTreeMap, DefaultMemory},
};
use derive_more::{Deref, DerefMut};
use std::{cell::RefCell, thread::LocalKey};

///
/// IndexStore
///

#[derive(Deref, DerefMut)]
pub struct IndexStore {
    pub data: BTreeMap<IndexKey, String>,
}

impl IndexStore {
    #[must_use]
    pub fn init(memory: DefaultMemory) -> Self {
        Self {
            data: BTreeMap::init(memory),
        }
    }
}

///
/// IndexStoreLocal
///

pub type IndexStoreLocal = &'static LocalKey<RefCell<IndexStore>>;
