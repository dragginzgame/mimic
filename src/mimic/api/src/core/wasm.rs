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

// get_wasm
pub fn get_wasm(path: &str) -> Result<&'static [u8], Error> {
    WasmManager::get_wasm(path).map_err(Error::from)
}

// info
pub fn info() -> Result<Vec<(String, usize)>, Error> {
    WasmManager::info().map_err(Error::from)
}
