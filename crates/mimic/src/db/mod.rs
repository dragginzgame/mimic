pub mod executor;
pub mod query;
pub mod response;
pub mod service;
pub mod store;

use crate::{
    Error,
    core::{serialize::SerializeError, traits::EntityKind, validate::ValidationError},
    db::{
        executor::{DeleteExecutor, ExecutorError, LoadExecutor, SaveExecutor},
        query::QueryError,
        response::ResponseError,
        store::{DataStoreRegistryLocal, IndexStoreRegistryLocal, StoreError},
    },
};
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
/// entry point into the whole db crate
///

#[derive(Clone, Copy)]
pub struct Db {
    data: DataStoreRegistryLocal,
    index: IndexStoreRegistryLocal,
}

impl Db {
    #[must_use]
    pub const fn new(data: DataStoreRegistryLocal, index: IndexStoreRegistryLocal) -> Self {
        Self { data, index }
    }

    #[must_use]
    pub const fn load<E: EntityKind>(&self) -> LoadExecutor<E> {
        LoadExecutor::new(self.data, self.index)
    }

    // save
    // needed because a Query can be passed over the wire without a SaveMode
    // but locally, the replace/update/create shortcut methods are preferred
    #[must_use]
    pub const fn save<E: EntityKind>(&self) -> SaveExecutor<E> {
        SaveExecutor::new(self.data, self.index)
    }

    #[must_use]
    pub const fn delete<E: EntityKind>(&self) -> DeleteExecutor<E> {
        DeleteExecutor::new(self.data, self.index)
    }

    ///
    /// High level, common shortcuts
    ///

    pub fn create<E: EntityKind>(&self, entity: E) -> Result<E, Error> {
        self.save::<E>().create(entity)
    }

    pub fn replace<E: EntityKind>(&self, entity: E) -> Result<E, Error> {
        self.save::<E>().replace(entity)
    }

    pub fn update<E: EntityKind>(&self, entity: E) -> Result<E, Error> {
        self.save::<E>().update(entity)
    }

    pub fn create_view<E: EntityKind>(&self, view: E::View) -> Result<E::View, Error> {
        self.save::<E>().create_view::<E::View>(view)
    }

    pub fn replace_view<E: EntityKind>(&self, view: E::View) -> Result<E::View, Error> {
        self.save::<E>().replace_view::<E::View>(view)
    }

    pub fn update_view<E: EntityKind>(&self, view: E::View) -> Result<E::View, Error> {
        self.save::<E>().update_view::<E::View>(view)
    }
}
