use crate::node::{Def, MacroNode, ValidateNode, VisitableNode};
use candid::CandidType;
use serde::Serialize;

///
/// Canister
///

#[derive(CandidType, Clone, Debug, Serialize)]
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
