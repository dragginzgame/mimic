use crate::{
    core::{Key, traits::EntityKind},
    db::{executor::ExecutorError, hasher::xx_hash_u64},
    debug,
    ic::structures::{BTreeMap, DefaultMemory},
    schema::node::EntityIndex,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use icu::{impl_storable_bounded, impl_storable_unbounded};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fmt::{self, Display},
    {cell::RefCell, thread::LocalKey},
};

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

    // insert_index_entry
    // we pass in the actual index to look for uniqueness

    // insert_index_entry
    pub fn insert_index_entry(
        &mut self,
        index: &EntityIndex,
        index_key: IndexKey,
        key: Key,
    ) -> Result<(), ExecutorError> {
        let debug = false;

        if let Some(existing) = self.get(&index_key) {
            let mut updated = existing;

            if index.unique {
                if !updated.contains(&key) && !updated.is_empty() {
                    debug!(debug, "index.insert: unique violation at {index_key}");
                    return Err(ExecutorError::IndexViolation(index_key));
                }

                self.insert(index_key.clone(), IndexEntry::from(key));
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
            self.insert(index_key.clone(), IndexEntry::from(key));
            debug!(
                debug,
                "index.insert: created new entry {index_key} -> {key}"
            );
        }

        Ok(())
    }

    // remove_index_entry
    // remove_index_entry
    pub fn remove_index_entry(&mut self, index_key: &IndexKey, key: &Key) -> Option<IndexEntry> {
        let debug = false;

        if let Some(existing) = self.get(index_key) {
            let mut updated = existing;
            let removed = updated.remove(key);
            debug!(
                debug,
                "index.remove: removed {key} from {index_key} (was present? {removed})"
            );

            if updated.is_empty() {
                debug!(
                    debug,
                    "index.remove: entry at {index_key} is now empty — removing"
                );
                self.remove(index_key)
            } else {
                self.insert(index_key.clone(), updated.clone());
                debug!(
                    debug,
                    "index.remove: updated entry at {index_key} -> {updated:?}"
                );

                Some(updated)
            }
        } else {
            debug!(debug, "index.remove: no entry found for {index_key}");

            None
        }
    }

    pub fn range_with_prefix<'a>(
        &'a self,
        index_id: &'a IndexId,
        prefix: &'a [Key],
    ) -> impl Iterator<Item = (IndexKey, IndexEntry)> + 'a {
        self.range(
            IndexKey {
                index_id: index_id.clone(),
                keys: prefix.to_vec(),
            }..,
        )
        .take_while(move |entry| {
            let k = entry.key();
            k.index_id == *index_id && k.keys.starts_with(prefix)
        })
        .map(|entry| (entry.key().clone(), entry.value()))
    }
}

///
/// IndexStoreLocal
///

pub type IndexStoreLocal = &'static LocalKey<RefCell<IndexStore>>;

///
/// IndexId
///

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, CandidType, Serialize, Deserialize,
)]
pub struct IndexId {
    pub entity_hash: u64,
    pub fields: Vec<String>,
}

impl IndexId {
    #[must_use]
    pub fn new<E: EntityKind>(fields: &[&str]) -> Self {
        Self::from_path(E::PATH, fields)
    }

    pub fn from_path(path: &str, fields: &[&str]) -> Self {
        Self {
            entity_hash: xx_hash_u64(path),
            fields: fields.iter().map(ToString::to_string).collect(),
        }
    }

    #[must_use]
    pub fn max_storable() -> Self {
        Self::from_path(
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

impl Display for IndexId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} [{:?}])", self.entity_hash, self.fields)
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
    pub const STORABLE_MAX_SIZE: u32 = 512;

    #[must_use]
    pub fn build<E: EntityKind>(e: &E, fields: &[&str]) -> Option<Self> {
        // Pull the values from the entity
        let values = e.values().collect_all(fields);

        // Early exit: if any value is null or fails to convert into a key
        let mut keys = Vec::with_capacity(values.len());
        for v in values {
            match v.as_key() {
                Some(k) => keys.push(k),
                None => return None,
            }
        }

        Some(Self {
            index_id: IndexId::new::<E>(fields),
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

#[derive(CandidType, Clone, Debug, Deref, DerefMut, Deserialize, Serialize)]
pub struct IndexEntry(HashSet<Key>);

impl IndexEntry {
    #[must_use]
    pub fn from_key(key: Key) -> Self {
        let mut set = HashSet::with_capacity(1);
        set.insert(key);

        Self(set)
    }

    #[must_use]
    pub fn insert(&mut self, key: Key) -> bool {
        self.0.insert(key)
    }

    /// Checks if the set contains the key.
    #[must_use]
    pub fn contains(&self, key: &Key) -> bool {
        self.0.contains(key)
    }

    /// Returns `true` if the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns number of keys in the entry.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns keys as a sorted `Vec<Key>` (useful for serialization/debug).
    #[must_use]
    pub fn to_sorted_vec(&self) -> Vec<Key> {
        let mut keys: Vec<_> = self.0.iter().copied().collect();
        keys.sort_unstable(); // uses Ord, fast
        keys
    }
}

impl From<Key> for IndexEntry {
    fn from(key: Key) -> Self {
        Self::from_key(key)
    }
}

impl FromIterator<Key> for IndexEntry {
    fn from_iter<I: IntoIterator<Item = Key>>(iter: I) -> Self {
        Self(HashSet::from_iter(iter))
    }
}

impl IntoIterator for IndexEntry {
    type Item = Key;
    type IntoIter = std::collections::hash_set::IntoIter<Key>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
}
