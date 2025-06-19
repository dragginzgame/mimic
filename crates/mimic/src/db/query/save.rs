use crate::{
    Error,
    ops::{
        serialize::{deserialize, serialize},
        traits::EntityKind,
    },
};
use candid::CandidType;
use derive_more::Display;
use serde::{Deserialize, Serialize};

///
/// SaveMode
///
/// Create  : will only insert a row if it's empty
/// Replace : will change the row regardless of what was there
/// Update  : will only change an existing row
///

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Display, Serialize)]
pub enum SaveMode {
    Create,
    Replace,
    Update,
}

///
/// SaveQueryBuilder
///

#[derive(Debug)]
pub struct SaveQueryBuilder {
    mode: SaveMode,
}

impl SaveQueryBuilder {
    // new
    #[must_use]
    pub const fn new(mode: SaveMode) -> Self {
        Self { mode }
    }

    // bytes
    #[must_use]
    pub fn bytes(self, bytes: &[u8]) -> SaveQuery {
        SaveQuery::new(self.mode, bytes)
    }

    // entity
    pub fn entity<E: EntityKind>(self, entity: E) -> Result<SaveQuery, Error> {
        let bytes = serialize(&entity)?;

        Ok(SaveQuery::new(self.mode, &bytes))
    }
}

///
/// SaveQuery
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct SaveQuery {
    pub mode: SaveMode,
    pub bytes: Vec<u8>,
}

impl SaveQuery {
    #[must_use]
    pub fn new(mode: SaveMode, bytes: &[u8]) -> Self {
        Self {
            mode,
            bytes: bytes.to_vec(),
        }
    }
}

impl<E: EntityKind> TryInto<SaveQueryTyped<E>> for SaveQuery {
    type Error = Error;

    fn try_into(self) -> Result<SaveQueryTyped<E>, Self::Error> {
        let entity = deserialize::<E>(&self.bytes)?;
        Ok(SaveQueryTyped::new(self.mode, entity))
    }
}

///
/// SaveQueryTypedBuilder
///

#[derive(Debug)]
pub struct SaveQueryTypedBuilder {
    mode: SaveMode,
}

impl SaveQueryTypedBuilder {
    // new
    #[must_use]
    pub const fn new(mode: SaveMode) -> Self {
        Self { mode }
    }

    // entity
    pub const fn entity<E: EntityKind>(self, entity: E) -> SaveQueryTyped<E> {
        SaveQueryTyped::new(self.mode, entity)
    }
}

///
/// SaveQueryTyped
///

#[derive(CandidType, Clone, Debug)]
pub struct SaveQueryTyped<E: EntityKind> {
    pub mode: SaveMode,
    pub entity: E,
}

impl<E> SaveQueryTyped<E>
where
    E: EntityKind,
{
    #[must_use]
    pub const fn new(mode: SaveMode, entity: E) -> Self {
        Self { mode, entity }
    }
}
