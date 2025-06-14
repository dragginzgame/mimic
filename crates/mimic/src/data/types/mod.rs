mod data;
mod entity;
mod index;
mod selector;

pub use data::*;
pub use entity::*;
pub use index::*;
pub use selector::*;

use candid::CandidType;
use icu::impl_storable_bounded;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

///
/// SortDirection
///

#[derive(CandidType, Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

///
/// SortKey
///

#[derive(
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct SortKey(pub Vec<(String, Option<String>)>);

impl SortKey {
    #[must_use]
    pub const fn new(parts: Vec<(String, Option<String>)>) -> Self {
        Self(parts)
    }

    /// creates an upper bound for the key by appending `~` to the last part's key.
    #[must_use]
    pub fn create_upper_bound(&self) -> Self {
        let mut parts = self.0.clone();

        if let Some((_, val)) = parts.last_mut() {
            *val = Some(match val {
                Some(s) => format!("{s}~"),
                None => "~".to_string(),
            });
        }

        Self(parts)
    }
}

impl Display for SortKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_parts: Vec<String> = self
            .0
            .iter()
            .map(|(path, key)| match key {
                Some(k) => format!("{path} ({k})"),
                None => format!("{path} (None)"),
            })
            .collect();

        write!(f, "[{}]", formatted_parts.join(", "))
    }
}

impl_storable_bounded!(SortKey, 256, false);

///
/// Where
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Where {
    pub matches: Vec<(String, String)>,
}
