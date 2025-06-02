mod db;
mod default;
mod validate;

use icu::{ic::export_candid, prelude::*};
use mimic::{Error as MimicError, prelude::*, query};
use test_schema::rarity::Rarity;

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
        .with(|reg| reg.with_store("test_schema::Index", |_| {}))
        .unwrap();

    log!(Log::Ok, "test: all tests passed successfully");
}

// rarity
#[update]
pub fn rarity() -> Result<Vec<Rarity>, MimicError> {
    perf!();

    let res = query_load!(
        query::load::<Rarity>()
            .all()
            .filter(|r| r.name.len() != 6)
            .search_field("name", "co")
            .sort([("level", SortDirection::Desc)])
    )?
    .entities();

    Ok(res)
}

export_candid!();
