use super::types::{DataKey, DataValue};
use crate::ic::structures::{memory::VirtualMemory, BTreeMap};
use derive_more::{Deref, DerefMut};

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
