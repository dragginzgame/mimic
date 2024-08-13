use crate::prelude::*;

///
/// Root Canister
///

#[canister(build = "root", initial_cycles = "50T", min_cycles = "15T")]
pub struct Root {}
