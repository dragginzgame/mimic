mod db;
mod default;
mod ops;
mod validate;

use icu::{ic::export_candid, prelude::*};
use mimic::{
    Error as MimicError,
    db::{
        query,
        store::{IndexEntry, IndexKey},
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
    ops::OpsTester::test();
    validate::ValidateTester::test();

    perf_start!();

    INDEX_REGISTRY
        .with(|reg| reg.with_store("test_design::schema::TestIndex", |_| {}))
        .unwrap();

    log!(Log::Ok, "test: all tests passed successfully");
}

// indexes
#[must_use]
#[query]
pub fn indexes() -> Vec<(IndexKey, IndexEntry)> {
    perf_start!();

    let res: Vec<(IndexKey, IndexEntry)> = TEST_INDEX.with_borrow(|i| i.iter().collect());

    res
}

#[update]
fn create_lots_simple() {
    use test_design::db::CreateBasic;
    const ROWS: u32 = 50;

    perf_start!();

    // insert rows
    for i in 0..ROWS {
        let e = CreateBasic::default();

        db!()
            .save()
            //       .debug()
            .execute(query::create().entity(e))
            .unwrap();

        if i % 10 == 0 {
            perf!("insert {i}");
        }
    }

    perf!("after inserts");

    // Retrieve the count from the store
    let count = db!()
        .load()
        .execute::<CreateBasic>(query::load().all())
        .unwrap()
        .count();

    debug!(true, "{count}");

    //assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
}

// create_lots_blob
#[update]
fn create_lots_blob() {
    use mimic::core::types::Blob;
    use test_design::db::CreateBlob;

    const ROWS: u32 = 2000;
    const BLOB_SIZE: usize = 1024 * 10;

    perf_start!();

    let bytes: Blob = vec![0u8; BLOB_SIZE].into();

    // insert rows
    for i in 1..=ROWS {
        let e = CreateBlob {
            bytes: bytes.clone(),
            ..Default::default()
        };
        let _res = db!()
            //       .debug()
            .save()
            .execute(query::create().entity(e))
            .unwrap();

        if i % 10 == 0 {
            //      icu::ic::println!("{:?}", res.0);
            perf!("insert {i}");
        }
    }

    // Retrieve the count from the store
    let count = db!()
        .load()
        .execute::<CreateBlob>(query::load().all())
        .unwrap()
        .count();

    let _ = count;

    //assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
}

// rarity
#[query]
pub fn rarity() -> Result<Vec<Rarity>, MimicError> {
    perf_start!();

    let res = db!()
        .load()
        .debug()
        .execute::<Rarity>(
            query::load()
                .all()
                .search_field("name", "co")
                .sort([("level", SortDirection::Desc)]),
        )?
        .entities();

    Ok(res)
}

export_candid!();
