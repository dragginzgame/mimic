use game::Error;
use ic_cdk::query;
use mimic::{core::wasm::WasmManager, prelude::*};

// start
mimic_start!("path/to/mimic.toml");

// init2
pub fn init2() -> Result<(), Error> {
    Ok(())
}

// init_async2
pub async fn init_async2() -> Result<(), Error> {
    Ok(())
}

// startup2
pub fn startup2() -> Result<(), Error> {
    Ok(())
}

// pre_upgrade
pub fn pre_upgrade2() -> Result<(), Error> {
    Ok(())
}

// post_upgrade2
pub fn post_upgrade2() -> Result<(), Error> {
    Ok(())
}


// end
mimic_end!();
