#![allow(clippy::type_complexity)]
use crate::{
    Error,
    core::{db::EntityKey, traits::EntityKind},
    db::{
        DataError,
        response::{EntityRow, ResponseError},
    },
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
    Keys(Vec<EntityKey>),
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
    // count
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn count(&self) -> u32 {
        self.0.len() as u32
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<EntityKey> {
        self.0.into_iter().next().map(|row| row.key)
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<EntityKey> {
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
    pub fn try_entity(self) -> Result<E, Error> {
        let res = self
            .0
            .into_iter()
            .next()
            .map(|row| row.entry.entity)
            .ok_or(ResponseError::EmptyCollection)
            .map_err(DataError::from)?;

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
pub struct LoadMap<T>(HashMap<EntityKey, T>);

impl<T> LoadMap<T> {
    // from_pairs
    pub fn from_pairs<I>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (EntityKey, T)>,
    {
        let map: HashMap<EntityKey, T> = pairs.into_iter().collect();

        Self(map)
    }

    // get
    pub fn get<K: Borrow<EntityKey>>(&self, k: K) -> Option<&T> {
        self.0.get(k.borrow())
    }

    // get_many
    pub fn get_many<K, I>(&self, keys: I) -> Vec<&T>
    where
        K: Borrow<EntityKey>,
        I: IntoIterator<Item = K>,
    {
        keys.into_iter()
            .filter_map(|k| self.0.get(k.borrow()))
            .collect()
    }
}
