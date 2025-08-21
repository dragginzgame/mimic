#![allow(clippy::type_complexity)]
use crate::{
    Error,
    core::{Key, traits::EntityKind},
    db::{DbError, response::ResponseError},
};
use derive_more::Deref;
use std::{borrow::Borrow, collections::HashMap};

///
/// LoadCollection
///

#[derive(Debug, Deref)]
pub struct LoadCollection<E: EntityKind>(pub Vec<(Key, E)>);

impl<E> LoadCollection<E>
where
    E: EntityKind,
{
    // count
    // not len, as it returns a u32 so could get confusing
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn count(&self) -> u32 {
        self.0.len() as u32
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///
    /// Key
    ///

    #[must_use]
    pub fn key(&self) -> Option<Key> {
        self.0.first().map(|(key, _)| *key)
    }

    pub fn try_key(&self) -> Result<Key, Error> {
        let key = self
            .key()
            .ok_or_else(|| ResponseError::NoRowsFound(E::PATH.to_string()))
            .map_err(DbError::from)?;

        Ok(key)
    }

    #[must_use]
    pub fn keys(&self) -> Vec<Key> {
        self.0.iter().map(|(key, _)| *key).collect()
    }

    pub fn keys_iter(self) -> impl Iterator<Item = Key> {
        self.0.into_iter().map(|(key, _)| key)
    }

    ///
    /// Entity
    ///

    #[must_use]
    pub fn entity(self) -> Option<E> {
        self.0.into_iter().next().map(|(_, e)| e)
    }

    pub fn try_entity(self) -> Result<E, Error> {
        let res = self
            .entity()
            .ok_or_else(|| ResponseError::NoRowsFound(E::PATH.to_string()))
            .map_err(DbError::from)?;

        Ok(res)
    }

    #[must_use]
    pub fn entities(self) -> Vec<E> {
        self.0.into_iter().map(|(_, e)| e).collect()
    }

    pub fn entities_iter(self) -> impl Iterator<Item = E> {
        self.0.into_iter().map(|(_, e)| e)
    }

    ///
    /// View
    ///

    #[must_use]
    pub fn view(self) -> Option<E::View> {
        self.entity().map(|e| e.to_view())
    }

    pub fn try_view(self) -> Result<E::View, Error> {
        self.try_entity().map(|e| e.to_view())
    }

    #[must_use]
    pub fn views(self) -> Vec<E::View> {
        self.entities().into_iter().map(|e| e.to_view()).collect()
    }

    pub fn views_iter(self) -> impl Iterator<Item = E::View> {
        self.entities().into_iter().map(|e| e.to_view())
    }

    ///
    /// Map
    ///

    #[must_use]
    pub fn into_map(self) -> LoadMap<E> {
        LoadMap::from_pairs(self.0)
    }
}

impl<E: EntityKind> IntoIterator for LoadCollection<E> {
    type Item = (Key, E);
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

#[derive(Debug)]
pub struct LoadMap<T>(HashMap<Key, T>);

impl<T> LoadMap<T> {
    #[must_use]
    pub const fn as_map(&self) -> &HashMap<Key, T> {
        &self.0
    }

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
