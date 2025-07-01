use crate::{
    core::{
        db::EntityKey,
        serialize::{SerializeError, deserialize, serialize},
        traits::Path,
    },
    db::store::{DataEntry, DataRow, Metadata},
};
use candid::CandidType;
use serde::{Serialize, de::DeserializeOwned};

///
/// EntityRow
/// same as DataRow but with a concrete Entity
///

#[derive(CandidType, Clone, Debug, Serialize)]
pub struct EntityRow<E>
where
    E: DeserializeOwned,
{
    pub key: EntityKey,
    pub entry: EntityEntry<E>,
}

impl<E> EntityRow<E>
where
    E: DeserializeOwned,
{
    pub const fn new(key: EntityKey, entry: EntityEntry<E>) -> Self {
        Self { key, entry }
    }
}

impl<E> TryFrom<DataRow> for EntityRow<E>
where
    E: DeserializeOwned,
{
    type Error = SerializeError;

    fn try_from(row: DataRow) -> Result<Self, Self::Error> {
        Ok(Self {
            key: row.key.into(),
            entry: row.entry.try_into()?,
        })
    }
}

///
/// EntityEntry
///

#[derive(CandidType, Clone, Debug, Serialize)]
pub struct EntityEntry<E>
where
    E: DeserializeOwned,
{
    pub entity: E,
    pub metadata: Metadata,
}

impl<E> TryFrom<DataEntry> for EntityEntry<E>
where
    E: DeserializeOwned,
{
    type Error = SerializeError;

    fn try_from(value: DataEntry) -> Result<Self, Self::Error> {
        let entity = deserialize::<E>(&value.bytes)?;

        Ok(Self {
            entity,
            metadata: value.metadata,
        })
    }
}

impl<E> TryFrom<EntityEntry<E>> for DataEntry
where
    E: Path + Serialize + DeserializeOwned,
{
    type Error = SerializeError;

    fn try_from(value: EntityEntry<E>) -> Result<Self, Self::Error> {
        let bytes = serialize::<E>(&value.entity)?;

        Ok(Self {
            bytes,
            path: E::path(),
            metadata: value.metadata,
        })
    }
}
