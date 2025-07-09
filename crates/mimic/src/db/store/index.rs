use crate::{
    core::{Key, Value, traits::EntityKind},
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

        match self.get(&index_key) {
            Some(existing) => {
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
            }
            None => {
                self.insert(index_key.clone(), IndexEntry::from(key));
                debug!(
                    debug,
                    "index.insert: created new entry {index_key} -> {key}"
                );
            }
        }

        Ok(())
    }

    // remove_index_entry
    // remove_index_entry
    pub fn remove_index_entry(&mut self, index_key: &IndexKey, key: &Key) -> Option<IndexEntry> {
        let debug = false;

        match self.get(index_key) {
            Some(existing) => {
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
            }
            None => {
                debug!(debug, "index.remove: no entry found for {index_key}");
                None
            }
        }
    }
}
///
/// IndexStoreLocal
///

pub type IndexStoreLocal = &'static LocalKey<RefCell<IndexStore>>;

///
/// IndexKey
///

#[derive(
    CandidType, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct IndexKey {
    pub index_id: u64, // hash of the entity path plus fields
    pub keys: Vec<Key>,
}

impl IndexKey {
    #[must_use]
    pub fn new<E: EntityKind>(e: &E, fields: &[&str]) -> Self {
        // Construct a canonical string like: "my::Entity::field1,field2"
        let full_key = format!("{}::{}", E::PATH, fields.join(","));

        // pull the values that match the index fields from the entity
        let keys = e
            .values()
            .collect_all(fields)
            .into_iter()
            .filter_map(Value::into_key)
            .collect();

        // debug!(true, "create index key - {:?}", fields);

        Self {
            index_id: xx_hash_u64(&full_key),
            keys,
        }
    }
}

impl Display for IndexKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} [{:?}])", self.index_id, self.keys)
    }
}

impl_storable_bounded!(IndexKey, 256, false);

///
/// IndexEntry
///

#[derive(CandidType, Clone, Debug, Deref, DerefMut, Serialize, Deserialize)]
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
