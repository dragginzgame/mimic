use crate::{
    api::ic::call::call,
    ic::{
        api::management_canister::{
            main::{CanisterInstallMode, CreateCanisterArgument, InstallCodeArgument, WasmModule},
            provisional::CanisterSettings,
        },
        format_cycles, id,
    },
    log, Log,
};
use candid::Principal;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("api error: {error}"))]
    Api { error: crate::api::Error },

    #[snafu(transparent)]
    Config { source: crate::core::config::Error },

    #[snafu(transparent)]
    Call { source: crate::api::ic::call::Error },

    #[snafu(transparent)]
    Mgmt { source: crate::api::ic::mgmt::Error },

    #[snafu(transparent)]
    Schema {
        source: crate::api::core::schema::Error,
    },
}

impl From<crate::api::Error> for Error {
    fn from(error: crate::api::Error) -> Self {
        Self::Api { error }
    }
}

///
/// create_canister
///

pub async fn create_canister(
    canister_path: &str,
    bytes: &[u8],
    parent_id: Principal,
) -> Result<Principal, Error> {
    let config = crate::core::config::get_config()?;

    //
    // controllers
    // default controllers + root
    //

    let mut controllers: Vec<Principal> = config.ic.controllers.clone();
    controllers.push(id());

    //
    // create canister
    //

    let canister_schema = crate::api::core::schema::canister(canister_path)?;
    let cycles = canister_schema.initial_cycles;
    let settings = Some(CanisterSettings {
        controllers: Some(controllers),
        ..Default::default()
    });

    let canister_id =
        crate::api::ic::mgmt::create_canister(CreateCanisterArgument { settings }, cycles).await?;

    //
    // install code
    //

    let install_arg = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: WasmModule::from(bytes),
        arg: ::candid::utils::encode_args((id(), parent_id)).expect("args encode"),
    };
    crate::api::ic::mgmt::install_code(install_arg).await?;

    //
    // call init_async
    //

    call::<_, (Result<(), crate::api::Error>,)>(canister_id, "init_async", ((),))
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
        format_cycles(cycles)
    );

    Ok(canister_id)
}
