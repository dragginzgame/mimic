mod db;
mod default;
mod sanitize;
mod validate;

use lib_ic::{log, update, Log};
use mimic::{api::Error, prelude::*};

// blank file so we get the default
mimic_start!("../config.toml");

// Startup
impl StartupHooks for StartupManager {}

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
