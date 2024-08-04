use crate::Error;
use candid::{CandidType, Principal};
use config::get_config;
use ic::{
    api::management_canister::{
        main::{CanisterInstallMode, InstallCodeArgument, WasmModule},
        provisional::CanisterSettings,
    },
    id, log, Log,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// CreateError
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum CreateError {
    #[snafu(transparent)]
    Config { source: config::Error },
}

///
/// create_canister
///

pub async fn create_canister(
    canister_path: &str,
    bytes: &[u8],
    parent_id: Principal,
) -> Result<Principal, Error> {
    let config = get_config().map_err(CreateError::from)?;

    //
    // controllers
    // default controllers + root
    //

    let mut controllers: Vec<Principal> = config.ic.controllers.clone();
    controllers.push(id());

    //
    // create canister
    //

    let canister_schema = crate::schema::canister(canister_path)?;
    let cycles = canister_schema.initial_cycles;
    let settings = Some(CanisterSettings {
        controllers: Some(controllers),
        ..Default::default()
    });

    let canister_id = super::mgmt::create_canister(
        ::ic::api::management_canister::main::CreateCanisterArgument { settings },
        cycles,
    )
    .await?;

    //
    // install code
    //

    let install_arg = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: WasmModule::from(bytes),
        arg: ::candid::utils::encode_args((id(), parent_id)).unwrap(),
    };
    crate::mgmt::install_code(install_arg).await?;

    //
    // call init_async
    //

    crate::call::<_, (Result<(), Error>,)>(canister_id, "init_async", ((),))
        .await?
        .0?;

    //
    // debug
    //

    #[allow(clippy::cast_precision_loss)]
    let bytes_fmt = bytes.len() as f64 / 1_000.0;
    log!(
        Log::Ok,
        "canister_create: {} created ({} KB) {} with {}",
        canister_path,
        bytes_fmt,
        canister_id,
        ::ic::format_cycles(cycles)
    );

    Ok(canister_id)
}
