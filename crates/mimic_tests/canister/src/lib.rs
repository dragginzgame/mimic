mod db;
mod default;
mod filter;
mod index;
mod ops;
mod validate;

use icu::{ic::export_candid, prelude::*, state::canister::Canister};
use mimic::{Error, db::query, prelude::*};
use test_design::{
    canister::filter::{Filterable, FilterableView},
    fixture::rarity::{Rarity, RarityView},
};

//
// INIT
//

icu_start_root!();
mimic_start!();

pub const CANISTERS: &[Canister] = &[];

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

// test
#[update]
pub fn test() {
    default::DefaultTester::test();
    db::DbTester::test();
    filter::delete::DeleteFilterTester::test();
    filter::index::IndexFilterTester::test();
    filter::load::LoadFilterTester::test();
    index::IndexTester::test();
    ops::OpsTester::test();
    validate::ValidateTester::test();

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

export_candid!();
