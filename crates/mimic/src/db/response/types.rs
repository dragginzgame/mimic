use crate::{
    core::{Key, deserialize, serialize, serialize::SerializeError, traits::EntityKind},
    db::store::{DataEntry, Metadata},
};

///
/// EntityRow
///

pub type EntityRow<E> = (Key, EntityEntry<E>);

///
/// EntityEntry
///

#[derive(Debug)]
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
