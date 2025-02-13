use crate::{
    schema::node::{Def, MacroNode, ValidateNode, VisitableNode},
    types::ErrorVec,
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
    pub name: String,
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

impl ValidateNode for Canister {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        if !self.name.is_case(Case::Snake) {
            errs.add("canister name must be snake case");
        }

        errs.result()
    }
}

impl VisitableNode for Canister {
    fn route_key(&self) -> String {
        self.def.path()
    }
}
