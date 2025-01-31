mod db;
mod default;
mod validate;

use mimic::{
    ic::{init, update},
    log, mimic_end, mimic_start, mimic_stores, Log,
};

mimic_start!();
mimic_stores!(STORE, 1);

#[init]
pub fn init() {
    mimic_init_schema();
    mimic_init_config();
}

// test
#[update]
pub fn test() {
    default::DefaultTester::test();
    db::DbTester::test();
    validate::ValidateTester::test();

    log!(Log::Ok, "test: all tests passed successfully");
}

mimic_end!();
