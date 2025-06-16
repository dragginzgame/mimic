mod data;
mod entity;
mod index;
mod selector;
mod sort_key;

pub use data::*;
pub use entity::*;
pub use index::*;
pub use selector::*;
pub use sort_key::*;

use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// SortDirection
///

#[derive(CandidType, Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

///
/// Where
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct Where {
    pub matches: Vec<(String, String)>,
}
