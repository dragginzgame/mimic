mod db;
mod default;
mod validate;

use mimic::{ic::update, log, prelude::*, Log};

// blank file so we get the default
mimic_start!("../mimic.toml");

// Startup
impl StartupHooks for StartupManager {}

// test
#[update]
pub fn test() {
    // default
    default::DefaultTester::test();

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
