use core_wasm::WasmManager;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    CoreWasm { source: core_wasm::Error },
}

// add_wasm
pub fn add_wasm(path: &'static str, wasm: &'static [u8]) -> Result<(), Error> {
    WasmManager::add_wasm(path, wasm).map_err(Error::from)
}

// get_wasm
pub fn get_wasm(path: &str) -> Result<&'static [u8], Error> {
    WasmManager::get_wasm(path).map_err(Error::from)
}

// info
pub fn info() -> Result<Vec<(String, usize)>, Error> {
    WasmManager::info().map_err(Error::from)
}
