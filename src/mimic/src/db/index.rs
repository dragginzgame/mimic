use crate::{
    db::types::CompositeKey,
    ic::structures::{BTreeMap, DefaultMemory},
};
use derive_more::{Deref, DerefMut};
use std::{cell::RefCell, thread::LocalKey};

///
/// Index
///

#[derive(Deref, DerefMut)]
pub struct Index {
    pub data: BTreeMap<CompositeKey, String>,
}

impl Index {
    // init
    #[must_use]
    pub fn init(memory: DefaultMemory) -> Self {
        Self {
            data: BTreeMap::init(memory),
        }
    }
}

///
/// IndexLocal
///

pub type IndexLocal = &'static LocalKey<RefCell<Index>>;
