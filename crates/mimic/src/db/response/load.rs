#![allow(clippy::type_complexity)]
use crate::{
    MimicError,
    core::{Key, traits::EntityKind},
    db::{
        DbError,
        response::{EntityRow, ResponseError},
    },
};
use derive_more::Deref;
use std::{borrow::Borrow, collections::HashMap};

///
/// LoadCollection
///

#[derive(Debug, Deref)]
pub struct LoadCollection<E: EntityKind>(pub Vec<EntityRow<E>>);

impl<E> LoadCollection<E>
where
    E: EntityKind,
{
    // len
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn len(&self) -> u32 {
        self.0.len() as u32
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<Key> {
        self.0.into_iter().next().map(|row| row.key)
    }

    // try_key
    pub fn try_key(self) -> Result<Key, MimicError> {
        let key = self
            .key()
            .ok_or(ResponseError::NoRowsFound)
            .map_err(DbError::from)?;

        Ok(key)
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<Key> {
        self.0.into_iter().map(|row| row.key).collect()
    }

    // map
    #[must_use]
    pub fn map(self) -> LoadMap<E> {
        LoadMap::from_pairs(self.0.into_iter().map(|row| (row.key, row.entry.entity)))
    }

    // entity
    #[must_use]
    pub fn entity(self) -> Option<E> {
        self.0.into_iter().next().map(|row| row.entry.entity)
    }

    // try_entity
    pub fn try_entity(self) -> Result<E, MimicError> {
        let res = self
            .entity()
            .ok_or(ResponseError::NoRowsFound)
            .map_err(DbError::from)?;

        Ok(res)
    }

    // entities
    #[must_use]
    pub fn entities(self) -> Vec<E> {
        self.0.into_iter().map(|row| row.entry.entity).collect()
    }

    // entities_iter
    pub fn entities_iter(self) -> impl Iterator<Item = E> {
        self.0.into_iter().map(|row| row.entry.entity)
    }

    // entity_row
    #[must_use]
    pub fn entity_row(self) -> Option<EntityRow<E>> {
        self.0.into_iter().next()
    }

    // try_entity_row
    pub fn try_entity_row(self) -> Result<EntityRow<E>, MimicError> {
        let row = self
            .entity_row()
            .ok_or(ResponseError::NoRowsFound)
            .map_err(DbError::from)?;

        Ok(row)
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
