mod db;
mod default;
mod validate;

use icu::{ic::export_candid, prelude::*};
use mimic::{
    Error as MimicError,
    db::{
        query,
        types::{IndexKey, IndexValue},
    },
    debug,
    prelude::*,
};
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

// indexes
#[must_use]
#[query]
pub fn indexes() -> Vec<(IndexKey, IndexValue)> {
    perf!();

    let res: Vec<(IndexKey, IndexValue)> = TEST_INDEX.with_borrow(|i| i.iter().collect());

    res
}

// create_lots_blob
#[update]
fn create_lots_blob() {
    perf!();

    use test_design::db::CreateBlob;
    const ROWS: u32 = 1000;
    const BLOB_SIZE: usize = 1024 * 10;

    // insert rows
    for i in 0..ROWS {
        let e = CreateBlob {
            bytes: vec![0u8; BLOB_SIZE].into(),
            ..Default::default()
        };
        //   query_save!()
        //     .debug()
        //        .execute(query::create().entity(e))
        //        .unwrap();

        if i % 10 == 0 {
            debug!(true, "created {i}");
        }
    }

    // Retrieve the count from the store
    let count = query_load!()
        .execute::<CreateBlob>(query::load().all())
        .unwrap()
        .count();

    debug!(true, "{count}");

    //assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
}

// rarity
#[query]
pub fn rarity() -> Result<Vec<Rarity>, MimicError> {
    perf!();

    let res = query_load!()
        .debug()
        .execute::<Rarity>(
            query::load()
                .all()
                .search_field("name", "co")
                .sort([("level", SortDirection::Asc)]),
        )?
        .entities();

    Ok(res)
}

export_candid!();
