mod db;
mod default;
mod validate;

use icu::{Log, ic::export_candid, log};
use mimic::{
    ic::{init, update},
    mimic_memory_manager, mimic_start,
};

//
// MIMIC
//

mimic_memory_manager!();
mimic_start!();

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

export_candid!();
