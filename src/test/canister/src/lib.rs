mod db;
mod default;
mod sanitize;
mod validate;

use ic::{log, update, Log};
use mimic::{prelude::*, Error};

// blank file so we get the default
mimic_start!("../config.toml");

// init2
pub const fn init2() -> Result<(), Error> {
    Ok(())
}

// init_async2
pub async fn init_async2() -> Result<(), Error> {
    Ok(())
}

// startup2
pub const fn startup2() -> Result<(), Error> {
    Ok(())
}

// pre_upgrade2
pub const fn pre_upgrade2() -> Result<(), Error> {
    Ok(())
}

// post_upgrade2
pub const fn post_upgrade2() -> Result<(), Error> {
    Ok(())
}

// test
#[update]
pub fn test() {
    // default
    default::DefaultTester::test();

    // sanitize
    sanitize::SanitizeTester::test();

    // validate
    validate::ValidateTester::test();

    // store
    DB.with(|db| {
        let tester = db::DbTester::new(db);
        tester.test();
    });

    log!(Log::Ok, "test: all tests passed successfully");
}

mimic_end!();
