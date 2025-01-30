mod db;
mod default;
mod validate;

use mimic::{ic::update, log, mimic_end, mimic_start, Log};

mimic_start!("../mimic.toml");

// test
#[update]
pub fn test() {
    default::DefaultTester::test();
    db::DbTester::test();
    validate::ValidateTester::test();

    log!(Log::Ok, "test: all tests passed successfully");
}

mimic_end!();
