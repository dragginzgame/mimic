mod data;
mod entity;
mod helper;
mod index;
mod selector;
mod sort_key;

pub use data::*;
pub use entity::*;
pub use helper::*;
pub use index::*;
pub use selector::*;
pub use sort_key::*;

use candid::CandidType;
use serde::{Deserialize, Serialize};

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
/// Where
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Where {
    pub matches: Vec<(String, String)>,
}
