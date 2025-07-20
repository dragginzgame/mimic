use crate::{
    core::{
        Key,
        traits::{EntityKind, IndexKind},
    },
    db::{executor::ExecutorError, hasher::xx_hash_u64},
    debug,
    ic::structures::{BTreeMap, DefaultMemory},
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
            let removed = updated.remove(&key);
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
    pub fn new(path: &str, fields: &[&str]) -> Self {
        Self {
            entity_hash: xx_hash_u64(path),
            fields: fields.iter().map(ToString::to_string).collect(),
        }
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
    pub fn build<I>(entity: &impl EntityKind) -> Option<Self>
    where
        I: IndexKind,
    {
        // Pull the values from the entity
        let values = entity.values().collect_all(I::FIELDS);

        // Early exit: if any value is null or fails to convert into a key
        let mut keys = Vec::with_capacity(values.len());
        for v in values {
            match v.as_key() {
                Some(k) => keys.push(k),
                None => return None,
            }
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
