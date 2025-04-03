use crate::schema::types::SortDirection;
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

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
