use crate::{
    SerializeError,
    data::store::{DataRow, DataValue, Metadata, SortKey},
    deserialize, serialize,
    traits::Path,
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
    pub key: SortKey,
    pub value: EntityValue<E>,
}

impl<E> EntityRow<E>
where
    E: DeserializeOwned,
{
    pub const fn new(key: SortKey, value: EntityValue<E>) -> Self {
        Self { key, value }
    }
}

impl<E> TryFrom<DataRow> for EntityRow<E>
where
    E: DeserializeOwned,
{
    type Error = SerializeError;

    fn try_from(row: DataRow) -> Result<Self, Self::Error> {
        Ok(Self {
            key: row.key,
            value: row.value.try_into()?,
        })
    }
}

impl<E> TryFrom<EntityRow<E>> for DataRow
where
    E: Path + Serialize + DeserializeOwned,
{
    type Error = SerializeError;

    fn try_from(row: EntityRow<E>) -> Result<Self, Self::Error> {
        Ok(Self {
            key: row.key,
            value: row.value.try_into()?,
        })
    }
}

///
/// EntityValue
///

#[derive(CandidType, Clone, Debug, Serialize)]
pub struct EntityValue<E>
where
    E: DeserializeOwned,
{
    pub entity: E,
    pub metadata: Metadata,
}

impl<E> TryFrom<DataValue> for EntityValue<E>
where
    E: DeserializeOwned,
{
    type Error = SerializeError;

    fn try_from(value: DataValue) -> Result<Self, Self::Error> {
        let entity = deserialize::<E>(&value.data)?;

        Ok(Self {
            entity,
            metadata: value.metadata,
        })
    }
}

impl<E> TryFrom<EntityValue<E>> for DataValue
where
    E: Path + Serialize + DeserializeOwned,
{
    type Error = SerializeError;

    fn try_from(value: EntityValue<E>) -> Result<Self, Self::Error> {
        let data = serialize::<E>(&value.entity)?;

        Ok(Self {
            data,
            path: E::path(),
            metadata: value.metadata,
        })
    }
}
