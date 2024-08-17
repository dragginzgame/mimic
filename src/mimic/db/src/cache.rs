use crate::types::{DataKey, DataValue};
use derive_more::{Deref, DerefMut};
use lib_time::now;
use std::collections::BTreeMap;

///
/// EntityCache
/// non-stable memory store
///

#[derive(Debug, Deref, DerefMut)]
pub struct EntityCache {
    #[deref]
    #[deref_mut]
    pub data: BTreeMap<DataKey, DataValue>,
    pub created: u64,
}

impl EntityCache {
    #[must_use]
    pub fn init() -> Self {
        Self {
            data: BTreeMap::new(),
            created: now(),
        }
    }
}
