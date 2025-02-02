use crate::schema::node::{ValidateNode, VisitableNode};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Def
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Def {
    pub module_path: String,
    pub ident: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub comments: String,
}

impl Def {
    // path
    // the path to the actual Type
    // ie. design::game::Rarity
    #[must_use]
    pub fn path(&self) -> String {
        format!("{}::{}", self.module_path, self.ident)
    }
}

impl ValidateNode for Def {}

impl VisitableNode for Def {}
