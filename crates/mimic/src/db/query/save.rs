use crate::{MimicError, core::traits::EntityKind, serialize::serialize};
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

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Display, Default, Serialize)]
pub enum SaveMode {
    #[default]
    Create,
    Replace,
    Update,
}

///
/// SaveQuery
///

#[derive(CandidType, Clone, Debug, Deserialize, Default, Serialize)]
pub struct SaveQuery {
    pub mode: SaveMode,
    pub bytes: Vec<u8>,
}

impl SaveQuery {
    #[must_use]
    pub fn new(mode: SaveMode) -> Self {
        Self {
            mode,
            ..Default::default()
        }
    }

    // from
    pub fn from<E: EntityKind>(mut self, input: impl Into<E>) -> Result<Self, MimicError> {
        let entity = input.into();
        self.bytes = serialize(&entity)?;

        Ok(self)
    }

    // from_bytes
    #[must_use]
    pub fn from_bytes(mut self, bytes: &[u8]) -> Self {
        self.bytes = bytes.to_vec();
        self
    }

    // from_entity
    pub fn from_entity<E: EntityKind>(mut self, entity: E) -> Result<Self, MimicError> {
        self.bytes = serialize(&entity)?;

        Ok(self)
    }
}
