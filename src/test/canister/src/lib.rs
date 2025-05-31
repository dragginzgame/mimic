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

    log!(Log::Ok, "test: all tests passed successfully");
}

// rarity
#[update]
pub fn rarity() -> Result<Vec<Rarity>, MimicError> {
    perf!();

    let query = query::load::<Rarity>()
        .all()
        .filter(|r| r.name.len() != 6)
        .search_field("name", "co")
        .sort([("level", SortDirection::Desc)]);

    let es = query.debug().execute(&DB)?.entities();

    Ok(es)
}

export_candid!();
