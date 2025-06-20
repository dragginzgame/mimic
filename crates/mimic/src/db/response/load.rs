#![allow(clippy::type_complexity)]
use crate::{
    Error,
    db::{
        DataError,
        response::ResponseError,
        types::{DataRow, EntityRow, SortKey},
    },
    ops::traits::EntityKind,
    types::Key,
};
use candid::CandidType;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap};

///
/// LoadResponse
///

#[derive(CandidType, Debug, Deserialize, Serialize)]
pub enum LoadResponse {
    Rows(Vec<DataRow>),
    Keys(Vec<SortKey>),
    Count(u32),
}

///
/// LoadCollection
///

#[derive(Debug)]
pub struct LoadCollection<E: EntityKind>(pub Vec<EntityRow<E>>);

impl<E> LoadCollection<E>
where
    E: EntityKind,
{
    // as_dyn
    #[must_use]
    pub fn as_dyn(self) -> LoadCollectionDyn {
        let data_rows: Vec<DataRow> = self
            .0
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect();

        LoadCollectionDyn(data_rows)
    }

    // count
    #[must_use]
    pub const fn count(&self) -> u32 {
        self.0.len() as u32
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<SortKey> {
        self.0.into_iter().next().map(|row| row.key)
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<SortKey> {
        self.0.into_iter().map(|row| row.key).collect()
    }

    // data_row
    #[must_use]
    pub fn data_row(self) -> Option<DataRow> {
        self.as_dyn().data_row()
    }

    // data_rows
    #[must_use]
    pub fn data_rows(self) -> Vec<DataRow> {
        self.as_dyn().data_rows()
    }

    // blob
    #[must_use]
    pub fn blob(self) -> Option<Vec<u8>> {
        self.as_dyn().blob()
    }

    // blobs
    #[must_use]
    pub fn blobs(self) -> Vec<Vec<u8>> {
        self.as_dyn().blobs()
    }

    // map
    #[must_use]
    pub fn map(self) -> LoadMap<E> {
        LoadMap::from_pairs(
            self.0
                .into_iter()
                .map(|row| (row.key.into(), row.value.entity)),
        )
    }

    // entity
    #[must_use]
    pub fn entity(self) -> Option<E> {
        self.0.into_iter().next().map(|row| row.value.entity)
    }

    // try_entity
    pub fn try_entity(self) -> Result<E, Error> {
        let res = self
            .0
            .into_iter()
            .next()
            .map(|row| row.value.entity)
            .ok_or(ResponseError::EmptyCollection)
            .map_err(DataError::from)?;

        Ok(res)
    }

    // entities
    #[must_use]
    pub fn entities(self) -> Vec<E> {
        self.0.into_iter().map(|row| row.value.entity).collect()
    }

    // entities_iter
    pub fn entities_iter(self) -> impl Iterator<Item = E> {
        self.0.into_iter().map(|row| row.value.entity)
    }

    // entity_row
    #[must_use]
    pub fn entity_row(self) -> Option<EntityRow<E>> {
        self.0.into_iter().next()
    }

    // entity_rows
    #[must_use]
    pub fn entity_rows(self) -> Vec<EntityRow<E>> {
        self.0
    }
}

impl<E: EntityKind> IntoIterator for LoadCollection<E> {
    type Item = EntityRow<E>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

///
/// LoadCollectionDyn
///

#[derive(CandidType, Debug, Deserialize, Serialize)]
pub struct LoadCollectionDyn(pub Vec<DataRow>);

impl LoadCollectionDyn {
    // count
    #[must_use]
    pub const fn count(&self) -> u32 {
        self.0.len() as u32
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<SortKey> {
        self.0.into_iter().next().map(|row| row.key)
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<SortKey> {
        self.0.into_iter().map(|row| row.key).collect()
    }

    // data_row
    #[must_use]
    pub fn data_row(self) -> Option<DataRow> {
        self.0.into_iter().next()
    }

    // try_data_row
    pub fn try_data_row(self) -> Result<DataRow, Error> {
        let res = self
            .0
            .into_iter()
            .next()
            .ok_or(ResponseError::EmptyCollection)
            .map_err(DataError::from)?;

        Ok(res)
    }

    // data_rows
    #[must_use]
    pub fn data_rows(self) -> Vec<DataRow> {
        self.0
    }

    // blob
    #[must_use]
    pub fn blob(self) -> Option<Vec<u8>> {
        self.0.into_iter().next().map(|row| row.value.bytes)
    }

    // blobs
    #[must_use]
    pub fn blobs(self) -> Vec<Vec<u8>> {
        self.0.into_iter().map(|row| row.value.bytes).collect()
    }
}

impl From<Vec<DataRow>> for LoadCollectionDyn {
    fn from(rows: Vec<DataRow>) -> Self {
        Self(rows)
    }
}

///
/// LoadMap
/// a HashMap indexed by id to provide an indexed alternative
/// to Vec<Row>
///

#[derive(Debug, Deref)]
pub struct LoadMap<T>(HashMap<Key, T>);

impl<T> LoadMap<T> {
    // from_pairs
    pub fn from_pairs<I>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (Key, T)>,
    {
        let map: HashMap<Key, T> = pairs.into_iter().collect();

        Self(map)
    }

    // get
    pub fn get<K: Borrow<Key>>(&self, k: K) -> Option<&T> {
        self.0.get(k.borrow())
    }

    // get_many
    pub fn get_many<K, I>(&self, keys: I) -> Vec<&T>
    where
        K: Borrow<Key>,
        I: IntoIterator<Item = K>,
    {
        keys.into_iter()
            .filter_map(|k| self.0.get(k.borrow()))
            .collect()
    }
}
