use crate::{
    core::traits::CanisterKind,
    db::{Db, store::DataKey},
    metrics::{EntityStorage, IndexMetrics, StorageReport, StoreMetrics},
};
use std::collections::BTreeMap;

/// Build storage snapshot and per-entity breakdown; enrich path names using id→path map.
///
/// Example (inside a canister):
/// ```ignore
/// #[query]
/// fn my_storage() -> Result<mimic::metrics::StorageReport, mimic::Error> {
///     Ok(mimic::interface::storage::storage_report(&db(), MIMIC_ENTITY_ID_PATH))
/// }
/// ```
#[must_use]
pub fn storage_report<C: CanisterKind>(
    db: &Db<C>,
    id_to_path: &[(u64, &'static str)],
) -> StorageReport {
    let mut data = Vec::new();
    let mut index = Vec::new();
    let mut entity_storage: Vec<EntityStorage> = Vec::new();

    db.with_data(|reg| {
        reg.for_each(|path, store| {
            data.push(StoreMetrics {
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

            let map: BTreeMap<u64, &str> = id_to_path.iter().copied().collect();

            for (entity_id, (count, mem)) in by_entity {
                let path_name = map.get(&entity_id).copied().unwrap_or("");
                entity_storage.push(EntityStorage {
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
            index.push(IndexMetrics {
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
