use crate::{
    common::utils::hash::hash_u64,
    core::{
        Key,
        traits::{EntityKind, IndexKind},
    },
    db::{executor::ExecutorError, store::DataKey},
    debug,
    ic::structures::{BTreeMap, DefaultMemory},
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, Display};
use icu::{impl_storable_bounded, impl_storable_unbounded};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{self, Display},
    {cell::RefCell, thread::LocalKey},
};

///
/// IndexStoreLocal
///

pub type IndexStoreLocal = &'static LocalKey<RefCell<IndexStore>>;

///
/// IndexStore
///

#[derive(Deref, DerefMut)]
pub struct IndexStore(BTreeMap<IndexKey, IndexEntry>);

impl IndexStore {
    #[must_use]
    pub fn init(memory: DefaultMemory) -> Self {
        Self(BTreeMap::init(memory))
    }

    /// Inserts the given entity into the index defined by `I`.
    /// - If `I::UNIQUE`, insertion will fail if a conflicting entry already exists.
    /// - If the entity is missing required fields for this index, insertion is skipped.
    pub fn insert_index_entry<I: IndexKind>(
        &mut self,
        entity: &impl EntityKind,
    ) -> Result<(), ExecutorError> {
        let debug = false;

        // Skip if index key can't be built (e.g. optional fields missing)
        let Some(index_key) = IndexKey::build::<I>(entity) else {
            return Ok(());
        };
        let key = entity.key();

        if let Some(existing) = self.get(&index_key) {
            let mut updated = existing;

            if I::UNIQUE {
                if !updated.contains(&key) && !updated.is_empty() {
                    debug!(debug, "index.insert: unique violation at {index_key}");
                    return Err(ExecutorError::IndexViolation(index_key));
                }

                self.insert(index_key.clone(), IndexEntry::new(I::FIELDS, key));
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
            self.insert(index_key.clone(), IndexEntry::new(I::FIELDS, key));
            debug!(
                debug,
                "index.insert: created new entry {index_key} -> {key}"
            );
        }

        Ok(())
    }

    // remove_index_entry
    pub fn remove_index_entry<I: IndexKind>(
        &mut self,
        entity: &impl EntityKind,
    ) -> Option<IndexEntry> {
        let debug = false;

        // Skip if index key can't be built (e.g. optional fields missing)
        let index_key = IndexKey::build::<I>(entity)?;
        let key = entity.key();

        if let Some(existing) = self.get(&index_key) {
            let mut updated = existing;
            let removed = updated.remove_key(&key);
            debug!(
                debug,
                "index.remove: removed {key} from {index_key} (was present? {removed})"
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

    pub fn resolve_data_keys<E: EntityKind>(
        &self,
        index_path: &str,
        index_fields: &[&str],
        prefix: &[Key],
    ) -> Vec<DataKey> {
        self.range_with_prefix(index_path, index_fields, prefix)
            .flat_map(|(_, entry)| entry.keys)
            .map(|key| DataKey::new::<E>(key))
            .collect()
    }

    pub fn range_with_prefix(
        &self,
        index_path: &str,
        index_fields: &[&str],
        prefix: &[Key],
    ) -> impl Iterator<Item = (IndexKey, IndexEntry)> {
        let index_id = IndexId::new(index_path, index_fields);

        self.range(
            IndexKey {
                index_id,
                keys: prefix.to_vec(),
            }..,
        )
        .take_while(move |entry| {
            let k = entry.key();
            k.index_id == index_id && k.keys.starts_with(prefix)
        })
        .map(|entry| (entry.key().clone(), entry.value()))
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
    pub fn new(path: &str, fields: &[&str]) -> Self {
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
    pub fn from_index<I: IndexKind>() -> Self {
        Self::new(I::PATH, I::FIELDS)
    }

    #[must_use]
    pub fn max_storable() -> Self {
        Self::new(
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

#[derive(
    CandidType, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct IndexKey {
    pub index_id: IndexId,
    pub keys: Vec<Key>,
}

impl IndexKey {
    // with five keys it's 235 bytes
    pub const STORABLE_MAX_SIZE: u32 = 256;

    #[must_use]
    pub fn build<I>(entity: &impl EntityKind) -> Option<Self>
    where
        I: IndexKind,
    {
        let mut keys = Vec::with_capacity(I::FIELDS.len());

        // get each value and convert to key
        for field in I::FIELDS {
            let value = entity.get_value(field)?;

            let key = value.as_key()?;
            keys.push(key);
        }

        Some(Self {
            index_id: IndexId::from_index::<I>(),
            keys,
        })
    }

    // max_storable
    #[must_use]
    pub fn max_storable() -> Self {
        Self {
            index_id: IndexId::max_storable(),
            keys: [
                Key::max_storable(),
                Key::max_storable(),
                Key::max_storable(),
                Key::max_storable(),
                Key::max_storable(),
            ]
            .to_vec(),
        }
    }
}

impl Display for IndexKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} [{:?}])", self.index_id, self.keys)
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
