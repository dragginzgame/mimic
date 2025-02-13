use crate::{
    schema::node::{Def, MacroNode, ValidateNode, VisitableNode},
    utils::case::{Case, Casing},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Canister
/// u128 cycles are easier to deal with
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Canister {
    pub def: Def,
}

impl Canister {
    // name
    // ie. game_config
    #[must_use]
    pub fn name(&self) -> String {
        self.def.ident.to_case(Case::Snake)
    }
}

impl MacroNode for Canister {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Canister {}

impl VisitableNode for Canister {
    fn route_key(&self) -> String {
        self.def.path()
    }
}
