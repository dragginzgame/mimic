use crate::{
    core::{Key, deserialize, serialize, serialize::SerializeError, traits::EntityKind},
    db::store::{DataEntry, DataRow, Metadata},
};
use candid::CandidType;
use serde::Serialize;

///
/// EntityRow
/// same as DataRow but with a concrete Entity
///

#[derive(CandidType, Clone, Debug, Serialize)]
pub struct EntityRow<E: EntityKind> {
    pub key: Key,
    pub entry: EntityEntry<E>,
}

impl<E: EntityKind> EntityRow<E> {
    pub const fn new(key: Key, entry: EntityEntry<E>) -> Self {
        Self { key, entry }
    }
}

impl<E: EntityKind> TryFrom<DataRow> for EntityRow<E> {
    type Error = SerializeError;

    fn try_from(row: DataRow) -> Result<Self, Self::Error> {
        Ok(Self {
            key: row.key.key(),
            entry: row.entry.try_into()?,
        })
    }
}

///
/// EntityEntry
///

#[derive(CandidType, Clone, Debug, Serialize)]
pub struct EntityEntry<E: EntityKind> {
    pub entity: E,
    pub metadata: Metadata,
}

impl<E: EntityKind> TryFrom<DataEntry> for EntityEntry<E> {
    type Error = SerializeError;

    fn try_from(value: DataEntry) -> Result<Self, Self::Error> {
        let entity = deserialize::<E>(&value.bytes)?;

        Ok(Self {
            entity,
            metadata: value.metadata,
        })
    }
}

impl<E: EntityKind> TryFrom<EntityEntry<E>> for DataEntry {
    type Error = SerializeError;

    fn try_from(value: EntityEntry<E>) -> Result<Self, Self::Error> {
        let bytes = serialize::<E>(&value.entity)?;

        Ok(Self {
            bytes,
            metadata: value.metadata,
        })
    }
}
