pub use crate::prelude::*;

///
/// Canister
///

#[canister]
pub struct Canister {}

///
/// TestDataStore
///

#[store(
    ident = "TEST_DATA_STORE",
    ty = "Data",
    canister = "Canister",
    memory_id = 20
)]
pub struct TestDataStore {}

///
/// TestIndexStore
///

#[store(
    ident = "TEST_INDEX_STORE",
    ty = "Index",
    canister = "Canister",
    memory_id = 21
)]
pub struct TestIndexStore {}

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
