use crate::schema::types::SortDirection;
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

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
