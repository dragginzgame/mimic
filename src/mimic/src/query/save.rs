use crate::{
    Error,
    db::types::SortKey,
    deserialize,
    traits::{EntityKind, EntityKindDyn},
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

#[derive(CandidType, Clone, Copy, Debug, Display, Serialize, Deserialize)]
pub enum SaveMode {
    Create,
    Replace,
    Update,
}

///
/// SaveQuery
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
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

///
/// SaveQueryPrepared
///

#[derive(Debug)]
pub struct SaveQueryPrepared {
    pub mode: SaveMode,
    pub entity: Box<dyn EntityKindDyn>,
}

impl SaveQueryPrepared {
    // new
    #[must_use]
    pub fn new(mode: SaveMode, entity: Box<dyn EntityKindDyn>) -> Self {
        Self { mode, entity }
    }
}

///
/// SaveResponse
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct SaveResponse {
    pub key: SortKey,
    pub created: u64,
    pub modified: u64,
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
    pub fn bytes<E: EntityKind + 'static>(self, bytes: &[u8]) -> Result<SaveQueryPrepared, Error> {
        let entity = deserialize::<E>(bytes)?;

        Ok(SaveQueryPrepared::new(self.mode, Box::new(entity)))
    }

    // entity
    pub fn entity<E: EntityKind + 'static>(self, entity: E) -> SaveQueryPrepared {
        SaveQueryPrepared::new(self.mode, Box::new(entity))
    }

    // entity_dyn
    #[must_use]
    pub fn entity_dyn(self, entity: Box<dyn EntityKindDyn>) -> SaveQueryPrepared {
        SaveQueryPrepared::new(self.mode, entity)
    }
}
