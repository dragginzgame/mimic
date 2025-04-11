use crate::{
    ic::serialize::{SerializeError, deserialize, serialize},
    impl_storable_unbounded,
    orm::{base::types::SortKey, traits::Path},
};
use candid::CandidType;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

///
/// STORAGE & API TYPES
///

///
/// DataRow
/// the data B-tree key and value pair
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DataRow {
    pub key: SortKey,
    pub value: DataValue,
}

impl DataRow {
    #[must_use]
    pub const fn new(key: SortKey, value: DataValue) -> Self {
        Self { key, value }
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
/// DataValue
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DataValue {
    pub data: Vec<u8>,
    pub path: String,
    pub metadata: Metadata,
}

impl_storable_unbounded!(DataValue);

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

///
/// Metadata
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub created: u64,
    pub modified: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_key_order() {
        let parts = vec![
            ("part1".to_string(), Some("alpha".to_string())),
            ("part2".to_string(), Some("gamma".to_string())),
        ];
        let data_key = SortKey::new(parts);
        let upper_bound_key = data_key.create_upper_bound();

        assert!(
            data_key < upper_bound_key,
            "The original key should be less than the upper bound key."
        );
    }

    #[test]
    fn test_empty_last_key() {
        let parts = vec![
            ("part1".to_string(), Some("alpha".to_string())),
            ("part2".to_string(), None), // Initially empty key
        ];
        let data_key = SortKey::new(parts);
        let upper_bound_key = data_key.create_upper_bound();

        assert!(
            upper_bound_key.0.last().unwrap().1.is_some(),
            "The last key should not be None after creating an upper bound."
        );
        assert_eq!(
            upper_bound_key.0.last().unwrap().1.as_deref(),
            Some("~"),
            "The last item in the empty key should be '~'."
        );
    }

    #[test]
    fn test_non_empty_last_key() {
        let parts = vec![
            ("part1".to_string(), Some("alpha".to_string())),
            ("part2".to_string(), Some("gamma".to_string())),
        ];
        let data_key = SortKey::new(parts);
        let upper_bound_key = data_key.create_upper_bound();

        assert_eq!(
            upper_bound_key.0.last().unwrap().1.as_deref(),
            Some("gamma~"),
            "The last item should be 'gamma~'."
        );
    }

    #[test]
    fn test_rarity_ordering() {
        let rarity_empty = SortKey::new(vec![("Rarity".to_string(), None)]);
        let rarity_with_key =
            SortKey::new(vec![("Rarity".to_string(), Some("123123".to_string()))]);
        let rarity_upper_bound = SortKey::new(vec![("Rarity".to_string(), Some("~".to_string()))]);

        assert!(
            rarity_empty < rarity_with_key,
            "Rarity(None) should be less than Rarity(Some('123123'))."
        );
        assert!(
            rarity_with_key < rarity_upper_bound,
            "Rarity(Some('123123')) should be less than Rarity(Some('~'))."
        );
    }
}
