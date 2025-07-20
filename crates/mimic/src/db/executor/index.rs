use crate::{
    core::traits::{EntityKind, IndexKind, IndexKindFn},
    db::{executor::ExecutorError, store::IndexStoreRegistryLocal},
};

///
/// IndexAction
///

pub enum IndexAction<'a, E: EntityKind> {
    Insert {
        entity: &'a E,
        registry: &'a IndexStoreRegistryLocal,
    },
    Remove {
        entity: &'a E,
        registry: &'a IndexStoreRegistryLocal,
    },
    Update {
        old: Option<&'a E>,
        new: &'a E,
        registry: &'a IndexStoreRegistryLocal,
    },
}

impl<E: EntityKind> IndexKindFn for IndexAction<'_, E> {
    type Error = ExecutorError;

    fn apply<I: IndexKind>(&mut self) -> Result<(), Self::Error> {
        match *self {
            IndexAction::Insert { entity, registry } => {
                let store = registry.with(|map| map.get_store::<I::Store>());
                store.with_borrow_mut(|store| {
                    store.insert_index_entry::<I>(entity)?;

                    Ok(())
                })?;
            }

            IndexAction::Remove { entity, registry } => {
                let store = registry.with(|map| map.get_store::<I::Store>());
                store.with_borrow_mut(|store| {
                    store.remove_index_entry::<I>(entity);

                    Ok(())
                })?;
            }

            IndexAction::Update { old, new, registry } => {
                let store = registry.with(|map| map.get_store::<I::Store>());
                store.with_borrow_mut(|store| {
                    store.insert_index_entry::<I>(new)?;

                    if let Some(old) = old {
                        store.remove_index_entry::<I>(old);
                    }

                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}
