use candid::Principal;
use ic::api::{
    call::RejectionCode,
    management_canister::main::{
        canister_status as ic_canister_status, create_canister as ic_create_canister,
        deposit_cycles as ic_deposit_cycles, install_code as ic_install_code, CanisterIdRecord,
        CanisterStatusResponse, CreateCanisterArgument, InstallCodeArgument,
    },
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("call rejected: {error}"))]
    CallRejected { error: String },
}

impl From<(RejectionCode, String)> for Error {
    fn from(error: (RejectionCode, String)) -> Self {
        Self::CallRejected { error: error.1 }
    }
}

// module_hash
pub async fn module_hash(canister_id: Principal) -> Result<Option<Vec<u8>>, Error> {
    let response = canister_status(canister_id).await?;

    Ok(response.module_hash)
}

///
/// WRAPPED MGMT FUNCTIONS
/// wrapped to make them easier to use, and to automatically convert the error
///

// canister_status
pub async fn canister_status(canister_id: Principal) -> Result<CanisterStatusResponse, Error> {
    let res = ic_canister_status(CanisterIdRecord { canister_id })
        .await?
        .0;

    Ok(res)
}

// create_canister
// wrapped so we don't have the CanisterIdRecord
pub async fn create_canister(
    arg: CreateCanisterArgument,
    cycles: u128,
) -> Result<Principal, Error> {
    let res = ic_create_canister(arg, cycles).await?.0;

    Ok(res.canister_id)
}

// deposit_cycles
pub async fn deposit_cycles(canister_id: Principal, cycles: u128) -> Result<(), Error> {
    ic_deposit_cycles(CanisterIdRecord { canister_id }, cycles).await?;

    Ok(())
}

// install_code
pub async fn install_code(arg: InstallCodeArgument) -> Result<(), Error> {
    ic_install_code(arg).await?;

    Ok(())
}
