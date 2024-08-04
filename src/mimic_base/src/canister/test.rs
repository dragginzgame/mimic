use crate::canister;
use mimic::orm::prelude::*;

///
/// Test Canister
///

#[canister(build = "test", initial_cycles = "5T", min_cycles = "5T")]
pub struct Test {}

pub mod store {
    use super::*;

    ///
    /// Test Data
    ///

    #[store(
        canister = "canister::test::Test",
        memory_id = 20,
        crud(load = "allow", save = "allow", delete = "allow")
    )]
    pub struct Data {}
}
