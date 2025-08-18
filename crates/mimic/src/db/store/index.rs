use crate::{
    MAX_INDEX_FIELDS,
    common::utils::hash::hash_u64,
    core::{Key, Value, traits::EntityKind},
    db::{
        executor::ExecutorError,
        store::{DataKey, StoreRegistry},
    },
    ic::structures::{BTreeMap, DefaultMemoryImpl, memory::VirtualMemory},
    schema::node::Index,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, Display};
use icu::{debug, impl_storable_bounded, impl_storable_unbounded};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{self, Display},
};

///
/// IndexStoreRegistry
///

#[derive(Deref, DerefMut)]
pub struct IndexStoreRegistry(StoreRegistry<IndexStore>);

impl IndexStoreRegistry {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(StoreRegistry::new())
    }
}

///
/// IndexStore
///

#[derive(Deref, DerefMut)]
pub struct IndexStore(BTreeMap<IndexKey, IndexEntry, VirtualMemory<DefaultMemoryImpl>>);

impl IndexStore {
    #[must_use]
    pub fn init(memory: VirtualMemory<DefaultMemoryImpl>) -> Self {
        Self(BTreeMap::init(memory))
    }

    /// Inserts the given entity into the index defined by `I`.
    /// - If `I::UNIQUE`, insertion will fail if a conflicting entry already exists.
    /// - If the entity is missing required fields for this index, insertion is skipped.
    pub fn insert_index_entry<E: EntityKind>(
        &mut self,
        entity: &E,
        index: &Index,
    ) -> Result<(), ExecutorError> {
        let debug = false;

        // Skip if index key can't be built (e.g. optional fields missing)
        let Some(index_key) = IndexKey::new(entity, index) else {
            return Ok(());
        };
        let key = entity.key();

        if let Some(existing) = self.get(&index_key) {
            let mut updated = existing;

            if index.unique {
                if !updated.contains(&key) && !updated.is_empty() {
                    return Err(ExecutorError::index_violation(E::PATH, index.fields));
                }

                self.insert(index_key.clone(), IndexEntry::new(index.fields, key));
                debug!(
                    debug,
                    "index.insert: unique index updated {index_key} -> {key}"
                );
            } else {
                let inserted = updated.insert(key);
                self.insert(index_key.clone(), updated);

                debug!(
                    debug,
                    "index.insert: added {key} to {index_key} (new? {inserted})"
                );
            }
        } else {
            self.insert(index_key.clone(), IndexEntry::new(index.fields, key));
            debug!(
                debug,
                "index.insert: created new entry {index_key} -> {key}"
            );
        }

        Ok(())
    }

    // remove_index_entry
    pub fn remove_index_entry(
        &mut self,
        entity: &impl EntityKind,
        index: &Index,
    ) -> Option<IndexEntry> {
        let debug = false;

        // Skip if index key can't be built (e.g. optional fields missing)
        let index_key = IndexKey::new(entity, index)?;
        let key = entity.key();

        if let Some(existing) = self.get(&index_key) {
            let mut updated = existing;
            let removed = updated.remove_key(&key);
            debug!(
                debug,
                "index.remove: removed {key} from {index_key} (was: {removed})"
            );

            if updated.is_empty() {
                debug!(debug, "index.remove: {index_key} is empty — removing");

                self.remove(&index_key)
            } else {
                self.insert(index_key.clone(), updated.clone());
                debug!(debug, "index.remove: updating {index_key} -> {updated:?}");

                Some(updated)
            }
        } else {
            debug!(debug, "index.remove: no entry found for {index_key}");

            None
        }
    }

    #[must_use]
    pub fn resolve_data_values<E: EntityKind>(
        &self,
        index: &Index,
        prefix: &[Value],
    ) -> Vec<DataKey> {
        self.iter_with_hashed_prefix::<E>(index, prefix)
            .flat_map(|(_, entry)| entry.keys.iter().copied().collect::<Vec<_>>())
            .map(|key| DataKey::new::<E>(key))
            .collect()
    }

    /// Internal: iterate entries for this index whose hashed_values start with the hashed prefix.
    fn iter_with_hashed_prefix<E: EntityKind>(
        &self,
        index: &Index,
        prefix: &[Value],
    ) -> impl Iterator<Item = (IndexKey, IndexEntry)> {
        let index_id = IndexId::new::<E>(index);
        let hashed_prefix_opt = Self::index_fingerprints(prefix); // Option<Vec<[u8;16]>>

        self.range(
            IndexKey {
                index_id,
                hashed_values: Vec::new(),
            }..,
        )
        .take_while(move |entry| entry.key().index_id == index_id)
        .filter(move |entry| {
            if let Some(ref hp) = hashed_prefix_opt {
                entry.key().hashed_values.starts_with(hp)
            } else {
                false // if prefix had None/Unit/Unsupported, no matches via index
            }
        })
        .map(|entry| (entry.key().clone(), entry.value()))
    }

    fn index_fingerprints(values: &[Value]) -> Option<Vec<[u8; 16]>> {
        // collects to Option<Vec<_>>: None if any element was non-indexable
        values.iter().map(Value::to_index_fingerprint).collect()
    }
}

///
/// IndexId
///

#[derive(
    Clone,
    Debug,
    Display,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    CandidType,
    Serialize,
    Deserialize,
)]
pub struct IndexId(u64);

impl IndexId {
    #[must_use]
    pub fn new<E: EntityKind>(index: &Index) -> Self {
        Self::from_path_and_fields(E::PATH, index.fields)
    }

    fn from_path_and_fields(path: &str, fields: &[&str]) -> Self {
        let mut buffer = Vec::new();

        // much more efficient than format
        buffer.extend_from_slice(path.as_bytes());
        for field in fields {
            buffer.extend_from_slice(field.as_bytes());
            buffer.extend_from_slice(b"|");
        }

        Self(hash_u64(&buffer))
    }

    #[must_use]
    pub fn max_storable() -> Self {
        Self::from_path_and_fields(
            "path::to::long::entity::name::Entity",
            &[
                "long_field_one",
                "long_field_two",
                "long_field_three",
                "long_field_four",
            ],
        )
    }
}

///
/// IndexKey
///

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IndexKey {
    pub index_id: IndexId,
    pub hashed_values: Vec<[u8; 16]>,
}

impl IndexKey {
    // currently works out at 166
    pub const STORABLE_MAX_SIZE: u32 = 180;

    #[must_use]
    pub fn new<E: EntityKind>(entity: &E, index: &Index) -> Option<Self> {
        let mut hashed_values = Vec::<[u8; 16]>::new();

        // get each value and convert to key
        for field in index.fields {
            let value = entity.get_value(field)?;
            let fp = value.to_index_fingerprint()?; // bail if any component is non-indexable

            hashed_values.push(fp);
        }

        Some(Self {
            index_id: IndexId::new::<E>(index),
            hashed_values,
        })
    }

    // max_storable
    #[must_use]
    pub fn max_storable() -> Self {
        Self {
            index_id: IndexId::max_storable(),
            hashed_values: (0..MAX_INDEX_FIELDS).map(|_| [u8::MAX; 16]).collect(),
        }
    }
}

impl Display for IndexKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}, values: {}",
            self.index_id,
            self.hashed_values.len()
        )
    }
}

impl_storable_bounded!(IndexKey, IndexKey::STORABLE_MAX_SIZE, false);

///
/// IndexEntry
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct IndexEntry {
    fields: Vec<String>,
    keys: HashSet<Key>,
}

impl IndexEntry {
    #[must_use]
    pub fn new(fields: &[&str], key: Key) -> Self {
        let mut key_set = HashSet::with_capacity(1);
        key_set.insert(key);

        Self {
            fields: fields.iter().map(ToString::to_string).collect(),
            keys: key_set,
        }
    }

    #[must_use]
    pub fn insert(&mut self, key: Key) -> bool {
        self.keys.insert(key)
    }

    /// Removes the key from the set.
    #[must_use]
    pub fn remove_key(&mut self, key: &Key) -> bool {
        self.keys.remove(key)
    }

    /// Checks if the set contains the key.
    #[must_use]
    pub fn contains(&self, key: &Key) -> bool {
        self.keys.contains(key)
    }

    /// Returns `true` if the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Returns number of keys in the entry.
    #[must_use]
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Returns keys as a sorted `Vec<Key>` (useful for serialization/debug).
    #[must_use]
    pub fn to_sorted_vec(&self) -> Vec<Key> {
        let mut keys: Vec<_> = self.keys.iter().copied().collect();
        keys.sort_unstable(); // uses Ord, fast
        keys
    }
}

impl_storable_unbounded!(IndexEntry);

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::traits::Storable;

    #[test]
    fn index_key_max_size_is_bounded() {
        let index_key = IndexKey::max_storable();
        let size = Storable::to_bytes(&index_key).len();

        println!("max serialized size = {size}");
        assert!(size <= IndexKey::STORABLE_MAX_SIZE as usize);
    }

    #[test]
    fn index_entry_round_trip() {
        let original = IndexEntry::new(&["a", "b"], Key::from(1u64));
        let encoded = Storable::to_bytes(&original);
        let decoded = IndexEntry::from_bytes(encoded);

        assert_eq!(original.fields, decoded.fields);
        assert_eq!(original.to_sorted_vec(), decoded.to_sorted_vec());
    }
}
