use crate::{
    MimicError,
    core::traits::EntityKind,
    serialize::{deserialize, serialize},
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

    // from
    pub fn from<E: EntityKind>(self, input: impl Into<E>) -> Result<SaveQuery, MimicError> {
        let entity = input.into();
        let bytes = serialize(&entity)?;

        Ok(SaveQuery::new(self.mode, &bytes))
    }

    // from_entity
    pub fn from_entity<E: EntityKind>(self, entity: E) -> Result<SaveQuery, MimicError> {
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
    type Error = MimicError;

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

    // from
    pub fn from<E: EntityKind>(self, input: impl Into<E>) -> SaveQueryTyped<E> {
        let entity = input.into();

        SaveQueryTyped::new(self.mode, entity)
    }

    // from_entity
    pub const fn from_entity<E: EntityKind>(self, entity: E) -> SaveQueryTyped<E> {
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
