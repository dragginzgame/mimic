mod db;
mod default;
mod filter;
mod ops;
mod validate;

use icu::{ic::export_candid, prelude::*};
use mimic::{MimicError, db::query, prelude::*};
use test_design::fixture::rarity::{Rarity, Rarity_View};

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
pub fn rarity() -> Result<Vec<Rarity_View>, MimicError> {
    perf_start!();

    let res = db!()
        .load()
        .debug()
        .execute::<Rarity>(
            query::load()
                .all()
                .with_filter(|f| {
                    f.or_filter_group(|f| {
                        f.filter("level", Cmp::Gtoe, 2)
                            .filter("level", Cmp::Ltoe, 4)
                    })
                    .or_filter_group(|f| f.filter("name", Cmp::Contains, "incon"))
                })
                .sort([("level", SortDirection::Desc)]),
        )?
        .entities();

    Ok(res.to_view())
}

export_candid!();
