use crate::schema::node::{Def, MacroNode, ValidateNode, VisitableNode};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Canister
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Canister {
    pub def: Def,
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
