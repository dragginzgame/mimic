use crate::{
    core::schema::{get_schema, Error as CoreSchemaError},
    log,
    orm::schema::node::Canister,
    Log,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

///
/// WASM_FILES
/// use Mutex to ensure thread safety for mutable access
///

pub static WASM_FILES: LazyLock<Mutex<HashMap<&'static str, &'static [u8]>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Schema { source: CoreSchemaError },

    #[snafu(display("mutex lock failed"))]
    LockFailed,

    #[snafu(display("schema canister not found for path {path}"))]
    PathNotFound { path: String },

    #[snafu(display("wasm not found for path {path}"))]
    WasmNotFound { path: String },
}

///
/// WasmManager
///

pub struct WasmManager {}

impl WasmManager {
    // get_wasm
    pub fn get_wasm(path: &str) -> Result<&'static [u8], Error> {
        let file = WASM_FILES
            .lock()
            .map_err(|_| Error::LockFailed)?
            .get(path)
            .copied()
            .ok_or_else(|| Error::WasmNotFound {
                path: path.to_string(),
            })?;

        Ok(file)
    }

    // add_wasm
    #[allow(clippy::cast_precision_loss)]
    pub fn add_wasm(path: &'static str, wasm: &'static [u8]) -> Result<(), Error> {
        // check if in schema
        let schema = get_schema()?;
        if schema.get_node::<Canister>(path).is_none() {
            return Err(Error::PathNotFound {
                path: path.to_string(),
            });
        }

        // add wasm
        WASM_FILES
            .lock()
            .map_err(|_| Error::LockFailed)?
            .insert(path, wasm);

        log!(
            Log::Ok,
            "add_wasm: {} ({:.2} KB)",
            path,
            wasm.len() as f64 / 1000.0
        );

        Ok(())
    }

    // info
    pub fn info() -> Result<Vec<(String, usize)>, Error> {
        let info: Vec<(String, usize)> = WASM_FILES
            .lock()
            .map_err(|_| Error::LockFailed)?
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.len()))
            .collect();

        Ok(info)
    }
}
