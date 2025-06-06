mod db;
mod default;
mod validate;

use icu::{ic::export_candid, prelude::*};
use mimic::{Error as MimicError, data::query, prelude::*};
use test_design::fixtures::Rarity;

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
    validate::ValidateTester::test();

    INDEX_REGISTRY
        .with(|reg| reg.with_store("test_design::schema::TestIndex", |_| {}))
        .unwrap();

    log!(Log::Ok, "test: all tests passed successfully");
}

// rarity
#[query]
pub fn rarity() -> Result<Vec<Rarity>, MimicError> {
    perf!();

    let res = query_load!()
        .debug()
        .execute(
            query::load::<Rarity>()
                .all()
                .filter(|r| r.name.len() != 6)
                .search_field("name", "co")
                .sort([("level", SortDirection::Desc)]),
        )?
        .entities();

    Ok(res)
}

export_candid!();
