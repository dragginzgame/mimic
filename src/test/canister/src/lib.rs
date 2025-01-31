mod db;
mod default;
mod validate;

use mimic::{
    ic::{init, update},
    log, mimic_end, mimic_memory_manager, mimic_start, mimic_stores, Log,
};

mimic_memory_manager!();
mimic_start!("../mimic.toml");
mimic_stores!(MEMORY_MANAGER, STORE, 1);

#[init]
pub fn init() {
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

mimic_end!();
