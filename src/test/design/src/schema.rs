pub use crate::prelude::*;

///
/// Canister
///

#[canister]
pub struct Canister {}

///
/// TestStore
///

#[store(
    ident = "TEST_STORE",
    ty = "Data",
    canister = "Canister",
    memory_id = 20
)]
pub struct TestStore {}

///
/// TestIndex
///

#[store(ident = "INDEX", ty = "Index", canister = "Canister", memory_id = 21)]
pub struct TestIndex {}

///
/// FixtureStore
///

#[store(
    ident = "FIXTURE_STORE",
    ty = "Data",
    canister = "Canister",
    memory_id = 22
)]
pub struct FixtureStore {}
