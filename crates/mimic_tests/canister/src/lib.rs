mod db;
mod default;
mod filter;
mod index;
mod ops;
mod validate;

use icu::{ic::export_candid, prelude::*};
use mimic::{Error, db::query, prelude::*};
use test_design::{
    canister::filter::{Filterable, FilterableView},
    fixture::rarity::{Rarity, RarityView},
    schema::TestDataStore,
    simple::relation::HasManyRelationView,
};

//
// INIT
//

icu_start_root!();
mimic_start!();

pub static WASMS: &[(CanisterType, &[u8])] = &[];

const fn icu_setup() {}

#[allow(clippy::unused_async)]
async fn icu_install() {
    mimic_init();
}

#[allow(clippy::unused_async)]
async fn icu_upgrade() {}

///
/// ENDPOINTS
///

pub fn clear_test_data_store() {
    // clear before each test
    crate::DATA_REGISTRY.with(|reg| {
        let _ = reg.with_store_mut(TestDataStore::PATH, |s| s.clear());
    });
}

// test
#[update]
pub fn test() {
    let tests: Vec<(&str, fn())> = vec![
        ("default", default::DefaultTester::test),
        ("db", db::DbTester::test),
        ("delete_filter", filter::delete::DeleteFilterTester::test),
        ("index_filter", filter::index::IndexFilterTester::test),
        ("load_filter", filter::load::LoadFilterTester::test),
        ("index", index::IndexTester::test),
        ("ops", ops::OpsTester::test),
        ("validate", validate::ValidateTester::test),
    ];

    // run tests
    for (name, test_fn) in tests {
        clear_test_data_store();

        println!("Running test: {name}");
        test_fn();
    }

    perf_start!();

    log!(Log::Ok, "test: all tests passed successfully");
}

//
// ENDPOINTS
//

// filterable
#[query]
pub fn filterable() -> Result<Vec<FilterableView>, Error> {
    let res = db!().load::<Filterable>().all()?.entities().to_view();

    Ok(res)
}

// rarity
#[query]
pub fn rarity() -> Result<Vec<RarityView>, Error> {
    perf_start!();

    let query = query::load()
        .filter(|f| {
            // (level >= 2 AND level <= 4) OR (name CONTAINS "ncon")
            (f.gte("level", 2) & f.lte("level", 4)) | f.contains("name", "ncon")
        })
        .sort(|s| s.desc("level"));

    let rarities = db!().load::<Rarity>().debug().execute(&query)?.views();

    Ok(rarities)
}

// pass_in_many
#[query]
pub fn pass_in_many(has: HasManyRelationView) -> u32 {
    has.a_ids.len() as u32
}

export_candid!();
