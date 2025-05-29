use crate::schema::types::SortDirection;
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

///
/// Search
/// text-based searching for UI filters
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum Search {
    All(String),
    Fields(Vec<(String, String)>),
}

impl Search {
    #[must_use]
    pub fn all<T: Into<String>>(text: T) -> Self {
        Self::All(text.into())
    }

    #[must_use]
    pub fn fields<I, K, V>(search: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let pairs = search
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        Self::Fields(pairs)
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
