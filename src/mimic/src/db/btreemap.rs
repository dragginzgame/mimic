use crate::ic::structures::DefaultMemory;
use derive_more::{Deref, DerefMut};
use ic_stable_structures::{btreemap::BTreeMap as WrappedBTreeMap, Storable};

///
/// BTreeMap
/// a wrapper around BTreeMap that uses the default VirtualMemory
///

#[derive(Deref, DerefMut)]
pub struct BTreeMap<K, V>
where
    K: Storable + Ord + Clone,
    V: Storable,
{
    data: WrappedBTreeMap<K, V, DefaultMemory>,
}

impl<K, V> BTreeMap<K, V>
where
    K: Storable + Ord + Clone,
    V: Storable,
{
    #[must_use]
    pub fn init(memory: DefaultMemory) -> Self {
        Self {
            data: WrappedBTreeMap::init(memory),
        }
    }

    /// clear
    /// the original clear() method in the ic-stable-structures library
    /// couldn't be wrapped as it took ownership, so they made a new one
    pub fn clear(&mut self) {
        self.clear_new();
    }
}
