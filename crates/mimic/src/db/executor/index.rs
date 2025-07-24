use crate::{
    core::traits::{EntityKind, IndexKind, IndexKindFn, Path},
    db::{DbError, store::IndexStoreRegistryLocal},
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

impl<'a, E: EntityKind> IndexAction<'_, E> {
    const fn registry(&self) -> &IndexStoreRegistryLocal {
        match self {
            IndexAction::Insert { registry, .. }
            | IndexAction::Remove { registry, .. }
            | IndexAction::Update { registry, .. } => registry,
        }
    }
}

impl<E: EntityKind> IndexKindFn for IndexAction<'_, E> {
    type Error = DbError;

    fn apply<I: IndexKind>(&mut self) -> Result<(), Self::Error> {
        let store = self
            .registry()
            .with(|map| map.try_get_store(I::Store::PATH))?;

        match *self {
            IndexAction::Insert { entity, .. } => {
                store.with_borrow_mut(|store| store.insert_index_entry::<I>(entity))?;
            }

            IndexAction::Remove { entity, .. } => {
                store.with_borrow_mut(|store| store.remove_index_entry::<I>(entity));
            }

            IndexAction::Update { old, new, .. } => {
                store.with_borrow_mut(|store| {
                    store.insert_index_entry::<I>(new)?;

                    if let Some(old) = old {
                        store.remove_index_entry::<I>(old);
                    }

                    Ok::<_, DbError>(())
                })?;
            }
        }

        Ok(())
    }
}
