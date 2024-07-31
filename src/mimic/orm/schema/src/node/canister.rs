use crate::node::{Def, MacroNode, ValidateNode, VisitableNode};
use candid::CandidType;
use ic::TC;
use lib_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use types::ErrorVec;

//
// CYCLES
//

/// VALIDATE_MIN_CYCLES
pub const VALIDATE_MIN_CYCLES: u128 = 3 * TC;

///
/// Canister
/// u128 cycles are easier to deal with
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Canister {
    pub def: Def,
    pub initial_cycles: u128,
    pub min_cycles: u128,
    pub build: CanisterBuild,
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

        // cycles
        if self.initial_cycles < VALIDATE_MIN_CYCLES {
            errs.add(format!(
                "initial_cycles cannot be less than the configured minimum {VALIDATE_MIN_CYCLES}",
            ));
        }
        if self.min_cycles < VALIDATE_MIN_CYCLES {
            errs.add(format!(
                "min_cycles cannot be less than the configured minimum {VALIDATE_MIN_CYCLES}",
            ));
        }

        errs.result()
    }
}

impl VisitableNode for Canister {
    fn route_key(&self) -> String {
        self.def.path()
    }
}

///
/// CanisterBuild
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum CanisterBuild {
    Basic(CanisterBuildBasic),
    Root,
    Test,
    User,
}

impl CanisterBuild {
    #[must_use]
    pub const fn is_auto_created(&self) -> bool {
        match self {
            Self::User => true,
            Self::Basic(basic) if !basic.replicated => true,
            _ => false,
        }
    }
}

///
/// CanisterBuildBasic
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct CanisterBuildBasic {
    pub replicated: bool,
}
