mod db;
mod default;
mod sanitize;
mod validate;

use api::Error;
use ic::{log, update, Log};

// init2
pub fn init2() -> Result<(), Error> {
    Ok(())
}

// init_async2
pub async fn init_async2() -> Result<(), Error> {
    Ok(())
}

// pre_upgrade2
pub fn pre_upgrade2() -> Result<(), Error> {
    Ok(())
}

// post_upgrade2
pub fn post_upgrade2() -> Result<(), Error> {
    Ok(())
}

// test
#[update]
pub fn test() {
    // cache
    //     let mut tester = cache::CacheTester::new(store);
    //     tester.test();

    // default
    default::DefaultTester::test();

    // sanitize
    sanitize::SanitizeTester::test();

    // validate
    validate::ValidateTester::test();

    // store
    //    let tester = db::DbTester::new(db);
    //    tester.test();

    log!(Log::Ok, "test: all tests passed successfully");
}
