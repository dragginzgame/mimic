use crate::{
    core::{Key, traits::EntityKind},
    db::store::StoreRegistry,
    export::icu::cdk::structures::{BTreeMap, DefaultMemoryImpl, memory::VirtualMemory},
};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use icu::impl_storable_bounded;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

///
/// DataStoreRegistry
///

#[derive(Deref, DerefMut)]
pub struct DataStoreRegistry(StoreRegistry<DataStore>);

impl DataStoreRegistry {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(StoreRegistry::new())
    }
}

///
/// DataStore
///

#[derive(Deref, DerefMut)]
pub struct DataStore(BTreeMap<DataKey, Vec<u8>, VirtualMemory<DefaultMemoryImpl>>);

impl DataStore {
    #[must_use]
    pub fn init(memory: VirtualMemory<DefaultMemoryImpl>) -> Self {
        Self(BTreeMap::init(memory))
    }

    pub fn memory_bytes(&self) -> u64 {
        self.iter()
            .map(|entry| u64::from(DataKey::STORABLE_MAX_SIZE) + entry.value().len() as u64)
            .sum()
    }
}

///
/// DataRow
///

pub type DataRow = (DataKey, Vec<u8>);

///
/// DataKey
///

#[derive(
    CandidType, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct DataKey {
    entity_id: u64,
    key: Key,
}

impl DataKey {
    pub const STORABLE_MAX_SIZE: u32 = 72;

    #[must_use]
    pub fn new<E: EntityKind>(key: impl Into<Key>) -> Self {
        Self {
            entity_id: E::ENTITY_ID,
            key: key.into(),
        }
    }

    #[must_use]
    pub const fn key(&self) -> Key {
        self.key
    }

    #[must_use]
    pub const fn max_storable() -> Self {
        Self {
            entity_id: u64::MAX,
            key: Key::max_storable(),
        }
    }
}

impl Display for DataKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} ({})", self.entity_id, self.key)
    }
}

impl From<DataKey> for Key {
    fn from(key: DataKey) -> Self {
        key.key()
    }
}

impl_storable_bounded!(DataKey, Self::STORABLE_MAX_SIZE, false);
