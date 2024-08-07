use candid::CandidType;
use db::{DataKey, DataRow, DataValue, Metadata};
use derive_more::{Deref, DerefMut};
use orm::types::SortDirection;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

///
/// QueryRow
/// a version of DataRow that can be passed back to the frontend
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct QueryRow {
    pub key: DataKey,
    pub value: QueryValue,
}

impl From<DataRow> for QueryRow {
    fn from(row: DataRow) -> Self {
        Self {
            key: row.key,
            value: row.value.into(),
        }
    }
}

impl<E> TryFrom<EntityRow<E>> for QueryRow
where
    E: Serialize + DeserializeOwned,
{
    type Error = orm::Error;

    fn try_from(row: EntityRow<E>) -> Result<Self, Self::Error> {
        Ok(Self {
            key: row.key,
            value: row.value.try_into()?,
        })
    }
}

///
/// QueryValue
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct QueryValue {
    pub data: Vec<u8>,
    pub metadata: Metadata,
}

impl From<DataValue> for QueryValue {
    fn from(value: DataValue) -> Self {
        Self {
            data: value.data,
            metadata: value.metadata,
        }
    }
}

impl<E> TryFrom<EntityValue<E>> for QueryValue
where
    E: Serialize + DeserializeOwned,
{
    type Error = orm::Error;

    fn try_from(value: EntityValue<E>) -> Result<Self, Self::Error> {
        let data = orm::serialize::<E>(&value.entity)?;

        Ok(Self {
            data,
            metadata: value.metadata,
        })
    }
}

///
/// EntityRow
/// same as QueryRow but with a concrete Entity
///

#[derive(CandidType, Clone, Debug, Serialize)]
pub struct EntityRow<E>
where
    E: DeserializeOwned,
{
    pub key: DataKey,
    pub value: EntityValue<E>,
}

impl<E> TryFrom<DataRow> for EntityRow<E>
where
    E: DeserializeOwned,
{
    type Error = orm::Error;

    fn try_from(row: DataRow) -> Result<Self, Self::Error> {
        Ok(Self {
            key: row.key,
            value: row.value.try_into()?,
        })
    }
}

impl<E> TryFrom<QueryRow> for EntityRow<E>
where
    E: DeserializeOwned,
{
    type Error = orm::Error;

    fn try_from(row: QueryRow) -> Result<Self, Self::Error> {
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
    type Error = orm::Error;

    fn try_from(value: DataValue) -> Result<Self, Self::Error> {
        let entity = orm::deserialize::<E>(&value.data)?;

        Ok(Self {
            entity,
            metadata: value.metadata,
        })
    }
}

impl<E> TryFrom<QueryValue> for EntityValue<E>
where
    E: DeserializeOwned,
{
    type Error = orm::Error;

    fn try_from(value: QueryValue) -> Result<Self, Self::Error> {
        let entity = orm::deserialize::<E>(&value.data)?;

        Ok(Self {
            entity,
            metadata: value.metadata,
        })
    }
}

///
/// LoadRequest
/// (from the front end, so no generics)
///
/// entity : Entity path
/// format : the format you want the results in (Rows or Count)
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct LoadRequest {
    pub entity: String,
    pub method: LoadMethod,
    pub offset: u32,
    pub limit: Option<u32>,
    pub filter: Option<Filter>,
    pub order: Option<Order>,
    pub format: LoadFormat,
}

///
/// LoadFormat
///
/// a variant that specifies the format the LoadResponse should be in
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum LoadFormat {
    Rows,
    Count,
}

///
/// LoadMethod
///
/// Some of these variants, ie. All will return too much data
/// for the CRUD interface, but are handy regardless
///
/// All    : no sort key prefix, only works with top-level Sort Keys,
///          will probably not work if used on nested entities
/// One    : returns one row by composite key
/// Many   : returns many rows (from many composite keys)
/// Prefix : like all but we're asking for the composite key prefix
///          so Pet (Character=1) will return the Pets from Character 1
/// Range  : user-defined range, ie. Item=1000 Item=1500
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub enum LoadMethod {
    #[default]
    All,
    One(Vec<String>),
    Many(Vec<Vec<String>>),
    Prefix(Vec<String>),
    Range(Vec<String>, Vec<String>),
}

///
/// LoadResponse
/// The variant that defines what format the results of a 564uest
/// will be returned in
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum LoadResponse {
    Rows(Vec<QueryRow>),
    Count(u32),
}

///
/// SaveRequest
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct SaveRequest {
    pub entity: String,
    pub data: Vec<u8>,
    pub action: SaveRequestAction,
}

///
/// SaveRequestAction
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum SaveRequestAction {
    Create,
    Update,
}

///
/// SaveResponse
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum SaveResponse {
    Create(CreateResponse),
    Update(UpdateResponse),
}

///
/// CreateResponse
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub row: QueryRow,
}

///
/// UpdateResponse
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub row: QueryRow,
}

///
/// DeleteRequest
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRequest {
    pub entity: String,
    pub key: Vec<String>,
}

///
/// DeleteResponse
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub keys: Vec<DataKey>,
}

///
/// Filter
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum Filter {
    All(String),
    Fields(Vec<(String, String)>),
}

impl From<String> for Filter {
    fn from(text: String) -> Self {
        Self::All(text)
    }
}

impl From<&str> for Filter {
    fn from(text: &str) -> Self {
        Self::All(text.to_string())
    }
}

impl From<Vec<(String, String)>> for Filter {
    fn from(search: Vec<(String, String)>) -> Self {
        Self::Fields(search)
    }
}

///
/// Order
///

#[derive(CandidType, Clone, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct Order(Vec<(String, SortDirection)>);

impl From<Vec<&str>> for Order {
    fn from(fields: Vec<&str>) -> Self {
        Self(
            fields
                .into_iter()
                .map(|field| (field.to_string(), SortDirection::Asc))
                .collect(),
        )
    }
}

impl From<Vec<(String, SortDirection)>> for Order {
    fn from(order: Vec<(String, SortDirection)>) -> Self {
        Self(order)
    }
}

impl From<&[(String, SortDirection)]> for Order {
    fn from(order: &[(String, SortDirection)]) -> Self {
        Self(order.to_vec())
    }
}
