use crate::prelude::*;

///
/// Canister
///

#[derive(CandidType, Clone, Debug, Serialize)]
pub struct Canister {
    pub def: Def,
    pub memory_min: u8,
    pub memory_max: u8,
}

impl MacroNode for Canister {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Canister {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();

        // store
        if self.memory_min > self.memory_max {
            err!(errs, "memory_min must be equal to or less than memory_max");
        }

        errs.result()
    }
}

impl VisitableNode for Canister {
    fn route_key(&self) -> String {
        self.def.path()
    }
}
