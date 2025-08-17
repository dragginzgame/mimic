pub mod executor;
pub mod query;
pub mod response;
pub mod service;
pub mod store;

use crate::{
    Error,
    core::{
        serialize::SerializeError,
        traits::{CanisterKind, EntityKind},
        validate::ValidationError,
    },
    db::{
        executor::{DeleteExecutor, ExecutorError, LoadExecutor, SaveExecutor},
        query::QueryError,
        response::ResponseError,
        store::{DataStoreRegistry, IndexStoreRegistry, StoreError},
    },
};
use std::{marker::PhantomData, thread::LocalKey};
use thiserror::Error as ThisError;

///
/// DbError
///

#[derive(Debug, ThisError)]
pub enum DbError {
    #[error(transparent)]
    ExecutorError(#[from] ExecutorError),

    #[error(transparent)]
    QueryError(#[from] QueryError),

    #[error(transparent)]
    ResponseError(#[from] ResponseError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    StoreError(#[from] StoreError),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),
}

///
/// Db
///
/// A handle to the set of stores registered for a specific canister domain.
///
/// - `C` is the [`CanisterKind`] (schema/domain marker).
/// - Entities that belong to this domain must implement [`CanisterScope<C>`].
///
/// The `Db` acts as the entry point for querying, saving, and deleting entities
/// within a single canister's store registry.
///

pub struct Db<C: CanisterKind> {
    data: &'static LocalKey<DataStoreRegistry>,
    index: &'static LocalKey<IndexStoreRegistry>,
    _marker: PhantomData<C>,
}

// Manual Copy + Clone implementations.
// Safe because Db only contains &'static LocalKey<_> handles,
// duplicating them does not duplicate the Rc contents.
impl<C: CanisterKind> Copy for Db<C> {}

impl<C: CanisterKind> Clone for Db<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: CanisterKind> Db<C> {
    #[must_use]
    pub const fn new(
        data: &'static LocalKey<DataStoreRegistry>,
        index: &'static LocalKey<IndexStoreRegistry>,
    ) -> Self {
        Self {
            data,
            index,
            _marker: PhantomData,
        }
    }

    pub fn with_data<R>(&self, f: impl FnOnce(&DataStoreRegistry) -> R) -> R {
        self.data.with(|reg| f(reg))
    }

    pub fn with_index<R>(&self, f: impl FnOnce(&IndexStoreRegistry) -> R) -> R {
        self.index.with(|reg| f(reg))
    }

    //
    // Low-level executors
    //

    /// Get a [`LoadExecutor`] for building and executing queries that read entities.
    #[must_use]
    pub const fn load<E>(&self) -> LoadExecutor<'_, E>
    where
        E: EntityKind<Canister = C>,
    {
        LoadExecutor::from_db(self)
    }

    /// Get a [`SaveExecutor`] for inserting or updating entities.
    ///
    /// Normally you will use the higher-level `create/replace/update` shortcuts instead.
    #[must_use]
    pub const fn save<E>(&self) -> SaveExecutor<'_, E>
    where
        E: EntityKind<Canister = C>,
    {
        SaveExecutor::from_db(self)
    }

    /// Get a [`DeleteExecutor`] for deleting entities by key or query.
    #[must_use]
    pub const fn delete<E>(&self) -> DeleteExecutor<'_, E>
    where
        E: EntityKind<Canister = C>,
    {
        DeleteExecutor::from_db(self)
    }

    //
    // High-level save shortcuts
    //

    pub fn create<E>(&self, entity: E) -> Result<E, Error>
    where
        E: EntityKind<Canister = C>,
    {
        self.save::<E>().create(entity)
    }

    pub fn replace<E>(&self, entity: E) -> Result<E, Error>
    where
        E: EntityKind<Canister = C>,
    {
        self.save::<E>().replace(entity)
    }

    pub fn update<E>(&self, entity: E) -> Result<E, Error>
    where
        E: EntityKind<Canister = C>,
    {
        self.save::<E>().update(entity)
    }

    pub fn create_view<E>(&self, view: E::View) -> Result<E::View, Error>
    where
        E: EntityKind<Canister = C>,
    {
        self.save::<E>().create_view::<E::View>(view)
    }

    pub fn replace_view<E>(&self, view: E::View) -> Result<E::View, Error>
    where
        E: EntityKind<Canister = C>,
    {
        self.save::<E>().replace_view::<E::View>(view)
    }

    pub fn update_view<E>(&self, view: E::View) -> Result<E::View, Error>
    where
        E: EntityKind<Canister = C>,
    {
        self.save::<E>().update_view::<E::View>(view)
    }
}
