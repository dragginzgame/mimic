use crate::{
    core::{types::EntityKey, value::IndexValue},
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
    pub fn insert_index_entry(
        &mut self,
        index: &EntityIndex,
        index_key: IndexKey,
        entity_key: EntityKey,
    ) -> Result<(), ExecutorError> {
        let debug = false;

        if let Some(existing) = self.get(&index_key) {
            if index.unique {
                if !existing.contains(&entity_key) && !existing.is_empty() {
                    debug!(debug, "index.insert: unique violation at {index_key}");

                    return Err(ExecutorError::IndexViolation(index_key));
                }

                // Unique, but no violation → overwrite or no-op
                self.insert(index_key.clone(), IndexEntry::from_key(entity_key.clone()));

                debug!(
                    debug,
                    "index.insert: unique index updated {index_key} -> {entity_key}"
                );
            } else {
                let mut updated = existing;
                updated.insert(entity_key.clone());
                self.insert(index_key.clone(), updated);

                debug!(debug, "index.insert: appended {entity_key} to {index_key}");
            }
        } else {
            self.insert(index_key.clone(), IndexEntry::from_key(entity_key.clone()));

            debug!(
                debug,
                "index.insert: created new entry {index_key} -> {entity_key}"
            );
        }

        Ok(())
    }

    // remove_index_entry
    pub fn remove_index_entry(
        &mut self,
        index_key: &IndexKey,
        entity_key: &EntityKey,
    ) -> Option<IndexEntry> {
        let debug = false;

        if let Some(mut existing) = self.get(index_key) {
            debug!(debug, "removing {entity_key} from index {index_key}");
            existing.remove(entity_key);

            if existing.is_empty() {
                debug!(
                    debug,
                    "index.remove: index {index_key} is now empty, removing key"
                );

                self.remove(index_key)
            } else {
                debug!(
                    debug,
                    "index.remove: updated index {index_key} = {existing:?}"
                );

                self.insert(index_key.clone(), existing.clone());

                Some(existing)
            }
        } else {
            None
        }
    }
}

///
/// IndexStoreLocal
///

pub type IndexStoreLocal = &'static LocalKey<RefCell<IndexStore>>;

///
/// IndexRow
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct IndexRow {
    pub key: IndexKey,
    pub entry: IndexEntry,
}

impl IndexRow {
    #[must_use]
    pub const fn new(key: IndexKey, entry: IndexEntry) -> Self {
        Self { key, entry }
    }
}

///
/// IndexKey
///

#[derive(
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct IndexKey {
    pub index_id: u64, // hash of the entity path plus fields
    pub values: Vec<IndexValue>,
}

impl IndexKey {
    // fields are passed in statically from the
    #[must_use]
    pub fn build(entity_path: &str, fields: &[&str], values: &[IndexValue]) -> Option<Self> {
        if fields.len() != values.len() {
            return None;
        }

        // Construct a canonical string like: "my::Entity::field1,field2"
        let full_key = format!("{entity_path}::{}", fields.join(","));

        Some(Self {
            index_id: xx_hash_u64(&full_key),
            values: values.to_vec(),
        })
    }
}

impl Display for IndexKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value_strs = self
            .values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "({} [{}])", self.index_id, value_strs)
    }
}

impl_storable_bounded!(IndexKey, 256, false);

///
/// IndexEntry
///

#[derive(CandidType, Clone, Debug, Default, Deref, DerefMut, Deserialize, Serialize)]
pub struct IndexEntry(pub HashSet<EntityKey>);

impl IndexEntry {
    #[must_use]
    pub fn from_key(key: EntityKey) -> Self {
        Self::from(vec![key])
    }
}

impl<K: Into<EntityKey>> From<Vec<K>> for IndexEntry {
    fn from(k: Vec<K>) -> Self {
        Self(k.into_iter().map(Into::into).collect())
    }
}

impl_storable_unbounded!(IndexEntry);
