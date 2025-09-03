use crate::{
    core::{Key, traits::CanisterKind},
    db::{Db, store::DataKey},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

///
/// StorageReport
/// Live storage snapshot report
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct StorageReport {
    /// Live storage inventory for data stores.
    pub storage_data: Vec<DataStoreSnapshot>,
    /// Live storage inventory for index stores.
    pub storage_index: Vec<IndexStoreSnapshot>,
    /// Live per-entity storage breakdown by store and entity path.
    pub entity_storage: Vec<EntitySnapshot>,
}

/// Store-level snapshot metrics.
#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct DataStoreSnapshot {
    pub path: String,
    pub entries: u64,
    pub min_key: Option<Key>,
    pub max_key: Option<Key>,
    pub memory_bytes: u64,
}

///
/// IndexStoreSnapshot
/// Index-store snapshot metrics
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct IndexStoreSnapshot {
    pub path: String,
    pub entries: u64,
    pub memory_bytes: u64,
}

///
/// EntitySnapshot
/// Per-entity storage breakdown across stores
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct EntitySnapshot {
    /// Store path (e.g., test_design::schema::TestDataStore)
    pub store: String,
    /// Entity path (e.g., test_design::canister::db::Index)
    pub path: String,
    /// Number of rows for this entity in the store
    pub entries: u64,
    /// Approximate bytes used (key + value)
    pub memory_bytes: u64,
}

/// Build storage snapshot and per-entity breakdown; enrich path names using id→path map
#[must_use]
pub fn storage_report<C: CanisterKind>(
    db: &Db<C>,
    id_to_path: &[(u64, &'static str)],
) -> StorageReport {
    // Build id→path map once, reuse across stores
    let id_map: std::collections::BTreeMap<u64, &str> = id_to_path.iter().copied().collect();
    let mut data = Vec::new();
    let mut index = Vec::new();
    let mut entity_storage: Vec<EntitySnapshot> = Vec::new();

    db.with_data(|reg| {
        reg.for_each(|path, store| {
            data.push(DataStoreSnapshot {
                path: path.to_string(),
                entries: store.len(),
                min_key: store.first_key_value().map(|(k, _)| k.into()),
                max_key: store.last_key_value().map(|(k, _)| k.into()),
                memory_bytes: store.memory_bytes(),
            });

            let mut by_entity: BTreeMap<u64, (u64, u64)> = BTreeMap::new();

            for entry in store.iter() {
                let dk = entry.key();
                let bytes_len = entry.value().len() as u64;
                let (count, mem) = by_entity.entry(dk.entity_id()).or_insert((0, 0));

                *count = count.saturating_add(1);
                *mem = mem.saturating_add(DataKey::entry_size_bytes(bytes_len));
            }

            for (entity_id, (count, mem)) in by_entity {
                let path_name = id_map.get(&entity_id).copied().unwrap_or("");
                entity_storage.push(EntitySnapshot {
                    store: path.to_string(),
                    path: path_name.to_string(),
                    entries: count,
                    memory_bytes: mem,
                });
            }
        });
    });

    db.with_index(|reg| {
        reg.for_each(|path, store| {
            index.push(IndexStoreSnapshot {
                path: path.to_string(),
                entries: store.len(),
                memory_bytes: store.memory_bytes(),
            });
        });
    });

    StorageReport {
        storage_data: data,
        storage_index: index,
        entity_storage,
    }
}
