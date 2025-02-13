mod db;
mod default;
mod validate;

use mimic::{
    ic::{init, update},
    log, mimic_end, mimic_memory_manager, mimic_start, Log,
};

mimic_memory_manager!(MEMORY_MANAGER);
mimic_start!("../mimic.toml");

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
