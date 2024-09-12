use crate::mgmt::{install_code, module_hash};
use candid::{CandidType, Principal};
use ic::{
    api::management_canister::main::{CanisterInstallMode, InstallCodeArgument, WasmModule},
    helper::get_wasm_hash,
    log, Log,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("wasm hash matches"))]
    WasmHashMatches,
}

/// upgrade_canister
pub async fn upgrade_canister(canister_id: Principal, bytes: &[u8]) -> Result<(), crate::Error> {
    // module_hash
    let module_hash = module_hash(canister_id).await?;
    if module_hash == Some(get_wasm_hash(bytes)) {
        Err(Error::WasmHashMatches)?;
    }

    // args
    let install_args = InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade(None),
        canister_id,
        wasm_module: WasmModule::from(bytes),
        arg: vec![],
    };
    install_code(install_args).await?;

    // debug
    #[allow(clippy::cast_precision_loss)]
    let bytes_fmt = bytes.len() as f64 / 1_000.0;
    log!(
        Log::Ok,
        "canister_upgrade: {} ({} KB) upgraded",
        canister_id,
        bytes_fmt,
    );

    Ok(())
}
