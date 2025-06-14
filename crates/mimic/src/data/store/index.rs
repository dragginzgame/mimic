use crate::{
    data::types::{IndexKey, IndexValue},
    ic::structures::{BTreeMap, DefaultMemory},
};
use derive_more::{Deref, DerefMut};
use std::{cell::RefCell, thread::LocalKey};

///
/// IndexStore
///

#[derive(Deref, DerefMut)]
pub struct IndexStore(BTreeMap<IndexKey, IndexValue>);

impl IndexStore {
    #[must_use]
    pub fn init(memory: DefaultMemory) -> Self {
        Self(BTreeMap::init(memory))
    }
}

///
/// IndexStoreLocal
///

pub type IndexStoreLocal = &'static LocalKey<RefCell<IndexStore>>;
