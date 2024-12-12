use crate::ic::structures::memory::VirtualMemory;
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
    data: WrappedBTreeMap<K, V, VirtualMemory>,
}

impl<K, V> BTreeMap<K, V>
where
    K: Storable + Ord + Clone,
    V: Storable,
{
    #[must_use]
    pub fn init(memory: VirtualMemory) -> Self {
        Self {
            data: WrappedBTreeMap::init(memory),
        }
    }

    /// keys
    pub fn keys(&self) -> impl Iterator<Item = K> + '_ {
        self.data.keys()
    }

    /// values
    pub fn values(&self) -> impl Iterator<Item = V> + '_ {
        self.data.values()
    }

    /// clear
    /// the original clear() method in the ic-stable-structures library
    /// couldn't be wrapped as it took ownership, so they made a new one
    pub fn clear(&mut self) {
        self.clear_new();
    }
}
