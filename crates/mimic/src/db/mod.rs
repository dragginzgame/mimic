pub mod executor;
pub mod query;
pub mod response;
pub mod service;
pub mod store;

use crate::{
    Error,
    core::{
        serialize::SerializeError,
        traits::{CanisterKind, CanisterScope, EntityKind},
        validate::ValidationError,
    },
    db::{
        executor::{DeleteExecutor, ExecutorError, LoadExecutor, SaveExecutor},
        query::QueryError,
        response::ResponseError,
        store::{DataStoreRegistryLocal, IndexStoreRegistryLocal, StoreError},
    },
};
use std::marker::PhantomData;
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
    data: DataStoreRegistryLocal,
    index: IndexStoreRegistryLocal,
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
    pub const fn new(data: DataStoreRegistryLocal, index: IndexStoreRegistryLocal) -> Self {
        Self {
            data,
            index,
            _marker: PhantomData,
        }
    }

    //
    // Low-level executors
    //

    /// Get a [`LoadExecutor`] for building and executing queries that read entities.
    #[must_use]
    pub const fn load<E>(&self) -> LoadExecutor<E>
    where
        E: EntityKind + CanisterScope<C>,
    {
        LoadExecutor::new(self.data, self.index)
    }

    /// Get a [`SaveExecutor`] for inserting or updating entities.
    ///
    /// Normally you will use the higher-level `create/replace/update` shortcuts instead.
    #[must_use]
    pub const fn save<E>(&self) -> SaveExecutor<E>
    where
        E: EntityKind + CanisterScope<C>,
    {
        SaveExecutor::new(self.data, self.index)
    }

    /// Get a [`DeleteExecutor`] for deleting entities by key or query.
    #[must_use]
    pub const fn delete<E>(&self) -> DeleteExecutor<E>
    where
        E: EntityKind + CanisterScope<C>,
    {
        DeleteExecutor::new(self.data, self.index)
    }

    //
    // High-level save shortcuts
    //

    pub fn create<E>(&self, entity: E) -> Result<E, Error>
    where
        E: EntityKind + CanisterScope<C>,
    {
        self.save::<E>().create(entity)
    }

    pub fn replace<E>(&self, entity: E) -> Result<E, Error>
    where
        E: EntityKind + CanisterScope<C>,
    {
        self.save::<E>().replace(entity)
    }

    pub fn update<E>(&self, entity: E) -> Result<E, Error>
    where
        E: EntityKind + CanisterScope<C>,
    {
        self.save::<E>().update(entity)
    }

    pub fn create_view<E>(&self, view: E::View) -> Result<E::View, Error>
    where
        E: EntityKind + CanisterScope<C>,
    {
        self.save::<E>().create_view::<E::View>(view)
    }

    pub fn replace_view<E>(&self, view: E::View) -> Result<E::View, Error>
    where
        E: EntityKind + CanisterScope<C>,
    {
        self.save::<E>().replace_view::<E::View>(view)
    }

    pub fn update_view<E>(&self, view: E::View) -> Result<E::View, Error>
    where
        E: EntityKind + CanisterScope<C>,
    {
        self.save::<E>().update_view::<E::View>(view)
    }
}
