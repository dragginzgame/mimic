use crate::{
    Error,
    core::{Key, deserialize, serialize, traits::EntityKind, validate::validate},
    db::{
        Db, DbError,
        executor::{Context, ExecutorError},
        query::{SaveMode, SaveQuery},
        store::DataKey,
    },
    metrics,
};
use icu::utils::time::now_secs;
use std::marker::PhantomData;

///
/// SaveExecutor
///

#[derive(Clone, Copy)]
pub struct SaveExecutor<'a, E: EntityKind> {
    db: &'a Db<E::Canister>,
    debug: bool,
    _marker: PhantomData<E>,
}

impl<'a, E: EntityKind> SaveExecutor<'a, E> {
    #[must_use]
    pub const fn from_db(db: &'a Db<E::Canister>) -> Self {
        Self {
            db,
            debug: false,
            _marker: PhantomData,
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    ///
    /// EXECUTION PREP
    ///

    const fn context(&self) -> Context<'_, E> {
        Context::new(self.db)
    }

    ///
    /// EXECUTION METHODS
    ///

    // response
    // a specific response used by the automated query endpoints that
    // we will improve int he future
    pub fn response(&self, query: &SaveQuery) -> Result<Key, Error> {
        let bytes: E = deserialize(&query.bytes)?;
        let key = self.save_entity(query.mode, bytes)?.key();

        Ok(key)
    }

    pub fn create(&self, entity: E) -> Result<E, Error> {
        let entity = self.save_entity(SaveMode::Create, entity)?;

        Ok(entity)
    }

    pub fn create_view<V>(&self, view: E::View) -> Result<E::View, Error> {
        let entity = E::from_view(view);
        let saved_view = self.create(entity)?.to_view();

        Ok(saved_view)
    }

    pub fn update(&self, entity: E) -> Result<E, Error> {
        let entity = self.save_entity(SaveMode::Update, entity)?;

        Ok(entity)
    }

    pub fn update_view<V>(&self, view: E::View) -> Result<E::View, Error> {
        let entity = E::from_view(view);
        let saved_view = self.update(entity)?.to_view();

        Ok(saved_view)
    }

    pub fn replace(&self, entity: E) -> Result<E, Error> {
        let entity = self.save_entity(SaveMode::Replace, entity)?;

        Ok(entity)
    }

    pub fn replace_view<V>(&self, view: E::View) -> Result<E::View, Error> {
        let entity = E::from_view(view);
        let saved_view = self.replace(entity)?.to_view();

        Ok(saved_view)
    }

    // execute
    // serializes the save query to pass to save_entity
    pub fn execute(&self, query: &SaveQuery) -> Result<E, Error> {
        let e: E = deserialize(&query.bytes)?;
        let entity = self.save_entity(query.mode, e)?;

        Ok(entity)
    }

    // save_entity
    fn save_entity(&self, mode: SaveMode, mut entity: E) -> Result<E, DbError> {
        let mut span = metrics::Span::<E>::new(metrics::ExecKind::Save);
        let key = entity.key();
        let ctx = self.context();

        // validate
        validate(&entity)?;

        // debug
        //   debug!(self.debug, "query.{mode}: {} ({key:?}) ", E::PATH);

        //
        // match save mode
        // on Update and Replace compare old and new data
        //

        let now = now_secs();
        let data_key = DataKey::new::<E>(key);
        let old_result = ctx.with_store(|store| store.get(&data_key))?;

        // did anything change?
        let old = match (mode, old_result) {
            (SaveMode::Create | SaveMode::Replace, None) => {
                entity.touch_created(now);

                None
            }

            (SaveMode::Update | SaveMode::Replace, Some(old_bytes)) => {
                let old = deserialize::<E>(&old_bytes)?;
                entity.touch_updated(now);

                Some(old)
            }

            // invalid
            (SaveMode::Create, Some(_)) => return Err(ExecutorError::KeyExists(data_key))?,
            (SaveMode::Update, None) => return Err(ExecutorError::KeyNotFound(data_key))?,
        };

        // now we can serialize
        let bytes = serialize(&entity)?;

        // replace indexes, fail if there are any unique violations
        self.replace_indexes(old.as_ref(), &entity)?;

        // insert data row
        ctx.with_store_mut(|store| store.insert(data_key.clone(), bytes))?;

        span.set_rows(1);
        Ok(entity)
    }

    // replace_indexes: two-phase (validate, then mutate) to avoid partial updates
    fn replace_indexes(&self, old: Option<&E>, new: &E) -> Result<(), DbError> {
        use crate::db::store::IndexKey;

        // Phase 1: validate uniqueness for all indexes without mutating
        for index in E::INDEXES {
            // Only check when we can compute the new key and the index is unique
            if index.unique
                && let Some(new_idx_key) = IndexKey::new(new, index)
            {
                let store = self.db.with_index(|reg| reg.try_get_store(index.store))?;
                let violates = store.with_borrow(|s| {
                    if let Some(existing) = s.get(&new_idx_key) {
                        let new_entity_key = new.key();
                        !existing.contains(&new_entity_key) && !existing.is_empty()
                    } else {
                        false
                    }
                });
                if violates {
                    // Count the unique violation just like the store-level check would have
                    crate::metrics::with_metrics_mut(|m| {
                        crate::metrics::record_unique_violation_for::<E>(m);
                    });
                    return Err(ExecutorError::index_violation(E::PATH, index.fields).into());
                }
            }
        }

        // Phase 2: apply changes (remove old, insert new) for each index
        for index in E::INDEXES {
            let store = self.db.with_index(|reg| reg.try_get_store(index.store))?;
            store.with_borrow_mut(|s| {
                if let Some(old) = old {
                    s.remove_index_entry(old, index);
                }
                s.insert_index_entry(new, index)?;
                Ok::<(), DbError>(())
            })?;
        }

        Ok(())
    }
}
