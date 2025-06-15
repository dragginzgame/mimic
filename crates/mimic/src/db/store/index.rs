use crate::{
    db::types::{IndexKey, IndexValue},
    def::types::Key,
    ic::structures::{BTreeMap, DefaultMemory},
    schema::node::EntityIndex,
};
use derive_more::{Deref, DerefMut};
use icu::{Log, log};
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
    ) {
        if let Some(mut existing) = self.get(&index_key) {
            if !existing.contains(&entity_key) && !existing.is_empty() {
                return Err(ExecutorError::IndexViolation(index_key));
            }

            log!(Log::Info, "adding {entity_key} into index {index_key}");

            existing.insert(entity_key);
        } else {
            log!(
                Log::Info,
                "inserting vec![{entity_key}] into index {index_key}"
            );

            self.insert(index_key, vec![entity_key].into());
        }
    }

    // remove_index_value
    pub fn remove_index_value(
        &mut self,
        index: &EntityIndex,
        index_key: &IndexKey,
        entity_key: &Key,
    ) {
        if let Some(mut existing) = self.get(index_key) {
            log!(Log::Info, "removing {entity_key} from index {index_key}");
            existing.remove(entity_key);

            if existing.is_empty() {
                self.remove(index_key);
            } else {
                self.insert(index_key.clone(), existing);
            }
        }
    }
}

///
/// IndexStoreLocal
///

pub type IndexStoreLocal = &'static LocalKey<RefCell<IndexStore>>;
