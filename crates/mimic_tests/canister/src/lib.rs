mod db;
mod default;
mod filter;
mod index;
mod ops;
mod validate;

use icu::{ic::export_candid, prelude::*};
use mimic::{MimicError, db::query, prelude::*};
use test_design::fixture::rarity::{Rarity, RarityView};

//
// INIT
//

mimic_start!();

#[init]
fn init() {
    mimic_init();
}

// test
#[update]
pub fn test() {
    default::DefaultTester::test();
    db::DbTester::test();
    filter::FilterTester::test();
    index::IndexTester::test();
    ops::OpsTester::test();
    validate::ValidateTester::test();

    perf_start!();

    INDEX_REGISTRY
        .with(|reg| reg.with_store("test_design::schema::TestIndex", |_| {}))
        .unwrap();

    log!(Log::Ok, "test: all tests passed successfully");
}

//
// ENDPOINTS
//

// rarity
#[query]
pub fn rarity() -> Result<Vec<RarityView>, MimicError> {
    perf_start!();

    let res = db!()
        .load()
        .debug()
        .execute::<Rarity>(
            query::load()
                .filter(|f| {
                    f.or_group(|f| f.filter("level", Cmp::Gte, 2).filter("level", Cmp::Lte, 4))
                        .or_group(|f| f.filter("name", Cmp::Contains, "incon"))
                })
                .sort([("level", SortDirection::Desc)]),
        )?
        .entities();

    Ok(res.to_view())
}

export_candid!();
