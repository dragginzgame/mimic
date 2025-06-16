use crate::{
    db::{
        executor::ExecutorError,
        types::{IndexKey, IndexValue},
    },
    debug,
    ic::structures::{BTreeMap, DefaultMemory},
    schema::node::EntityIndex,
    types::Key,
};
use derive_more::{Deref, DerefMut};
use std::{cell::RefCell, thread::LocalKey};

///
/// IndexStore
///

#[derive(Deref, DerefMut)]
pub struct IndexStore(BTreeMap<IndexKey, IndexValue>);

impl IndexStore {
    #[must_use]
    pub fn init(memory: DefaultMemory) -> Self {
        Self(BTreeMap::init(memory))
    }

    // insert_index_value
    pub fn insert_index_value(
        &mut self,
        index: &EntityIndex,
        index_key: IndexKey,
        entity_key: Key,
    ) -> Result<(), ExecutorError> {
        let debug = false;

        if let Some(existing) = self.get(&index_key) {
            if index.unique {
                if !existing.contains(&entity_key) && !existing.is_empty() {
                    debug!(debug, "index.insert: unique violation at {index_key}");

                    return Err(ExecutorError::IndexViolation(index_key));
                }

                // Unique, but no violation → overwrite or no-op
                self.insert(index_key.clone(), IndexValue::from_key(entity_key.clone()));

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
            self.insert(index_key.clone(), IndexValue::from_key(entity_key.clone()));

            debug!(
                debug,
                "index.insert: created new entry {index_key} -> {entity_key}"
            );
        }

        Ok(())
    }

    // remove_index_value
    pub fn remove_index_value(
        &mut self,
        index_key: &IndexKey,
        entity_key: &Key,
    ) -> Option<IndexValue> {
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
