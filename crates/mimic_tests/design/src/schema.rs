pub use crate::prelude::*;

///
/// Canister
///

#[canister(memory_min = 50, memory_max = 100)]
pub struct Canister {}

///
/// TestDataStore
///

#[store(
    ident = "TEST_DATA_STORE",
    ty = "Data",
    canister = "Canister",
    memory_id = 50
)]
pub struct TestDataStore {}

///
/// TestIndexStore
///

#[store(
    ident = "TEST_INDEX_STORE",
    ty = "Index",
    canister = "Canister",
    memory_id = 51
)]
pub struct TestIndexStore {}
