use crate::{
    store::types::{DataKey, DataRow},
    types::SortDirection,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

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
/// All    : no sort key prefix, only works with top-level Sort Keys,
///          will probably not work if used on nested entities
/// Only   : for entities that have no keys
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
    Only,
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
    Rows(Vec<DataRow>),
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
    pub row: DataRow,
}

///
/// UpdateResponse
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub row: DataRow,
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

impl Filter {
    #[must_use]
    pub const fn all(text: String) -> Self {
        Self::All(text)
    }

    #[must_use]
    pub const fn fields(search: Vec<(String, String)>) -> Self {
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
