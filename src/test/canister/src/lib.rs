mod db;
mod default;
mod validate;

use icu::{ic::export_candid, prelude::*};
use mimic::prelude_actor::*;

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
    validate::ValidateTester::test();

    log!(Log::Ok, "test: all tests passed successfully");
}

export_candid!();
