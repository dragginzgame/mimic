use crate::ic::call::call;
use candid::{CandidType, Principal};
use core_state::ChildIndexManager;
use core_wasm::WasmManager;
use lib_ic::{caller, format_cycles, log, println, Log};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use strum::Display;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Call { source: crate::ic::call::Error },

    #[snafu(display("invalid response: {response}"))]
    InvalidResponse { response: Response },

    #[snafu(transparent)]
    Canister { source: crate::ic::canister::Error },

    #[snafu(transparent)]
    Mgmt { source: crate::ic::mgmt::Error },

    #[snafu(transparent)]
    CreateCanister { source: crate::ic::create::Error },

    #[snafu(transparent)]
    UpgradeCanister { source: crate::ic::upgrade::Error },

    #[snafu(transparent)]
    CoreWasm { source: core_wasm::Error },
}

///
/// Request
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    pub kind: RequestKind,
}

impl Request {
    #[must_use]
    pub const fn new_canister_create(path: String) -> Self {
        Self {
            kind: RequestKind::CanisterCreate(CanisterCreate { path }),
        }
    }

    #[must_use]
    pub const fn new_canister_upgrade(canister_id: Principal, path: String) -> Self {
        Self {
            kind: RequestKind::CanisterUpgrade(CanisterUpgrade { canister_id, path }),
        }
    }

    #[must_use]
    pub const fn new_cycles(cycles: u128) -> Self {
        Self {
            kind: RequestKind::Cycles(Cycles { cycles }),
        }
    }
}

///
/// RequestKind
///

#[derive(CandidType, Clone, Debug, Display, Serialize, Deserialize)]
pub enum RequestKind {
    CanisterCreate(CanisterCreate),
    CanisterUpgrade(CanisterUpgrade),
    Cycles(Cycles), // cycles amount
}

///
/// CanisterCreate
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct CanisterCreate {
    pub path: String,
}

///
/// CanisterUpgrade
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct CanisterUpgrade {
    pub canister_id: Principal,
    pub path: String,
}

///
/// Cycles
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Cycles {
    pub cycles: u128,
}

///
/// Response
///

#[derive(CandidType, Clone, Debug, Display, Serialize, Deserialize)]
pub enum Response {
    CanisterCreate(Principal),
    CanisterUpgrade,
    Cycles,
}

///
/// RESPONSE (ROOT)
///

// response
pub async fn response(req: Request) -> Result<Response, Error> {
    // ::ic::println!("root response : {req:?}");

    match req.kind {
        RequestKind::CanisterCreate(kind) => response_create_canister(&kind.path).await,
        RequestKind::CanisterUpgrade(kind) => {
            response_upgrade_canister(kind.canister_id, &kind.path).await
        }
        RequestKind::Cycles(kind) => response_send_cycles(caller(), kind.cycles).await,
    }
}

// response_create_canister
async fn response_create_canister(path: &str) -> Result<Response, Error> {
    let bytes = WasmManager::get_wasm(path).map_err(Error::from)?;
    let new_canister_id = crate::ic::create::create_canister(path, bytes, caller()).await?;

    Ok(Response::CanisterCreate(new_canister_id))
}

// response_upgrade_canister
async fn response_upgrade_canister(canister_id: Principal, path: &str) -> Result<Response, Error> {
    let bytes = WasmManager::get_wasm(path).map_err(Error::from)?;
    crate::ic::upgrade::upgrade_canister(canister_id, bytes).await?;

    Ok(Response::CanisterUpgrade)
}

// response_send_cycles
async fn response_send_cycles(canister_id: Principal, cycles: u128) -> Result<Response, Error> {
    // actually send cycles
    crate::ic::mgmt::deposit_cycles(canister_id, cycles).await?;

    // debug
    let balance = crate::ic::canister::balance();
    log!(
        Log::Info,
        "root_send_cycles: sending {} cycles to {}, end balance: {}",
        format_cycles(cycles),
        canister_id,
        format_cycles(balance)
    );

    Ok(Response::Cycles)
}

///
/// REQUEST
/// all types of canister, but root just passes it to response
///

// request
pub async fn request(request: Request) -> Result<Response, Error> {
    println!("request: {request:?}");

    let root_canister_id = crate::ic::canister::root_id()?;
    let res = call::<_, (Result<Response, crate::ic::call::Error>,)>(
        root_canister_id,
        "response",
        (request,),
    )
    .await?
    .0?;

    Ok(res)
}

// request_canister_create
// create a Request and pass it to the request shared endpoint
pub async fn request_canister_create(canister_path: &str) -> Result<Principal, Error> {
    let req = Request::new_canister_create(canister_path.to_string());

    match request(req).await {
        Ok(response) => match response {
            Response::CanisterCreate(new_canister_id) => {
                // success, update child index
                ChildIndexManager::add_canister(new_canister_id, canister_path);

                Ok(new_canister_id)
            }
            _ => Err(Error::InvalidResponse { response })?,
        },
        Err(e) => Err(e),
    }
}

// request_canister_upgrade
pub async fn request_canister_upgrade(
    canister_id: Principal,
    canister_path: String,
) -> Result<(), Error> {
    let req = Request::new_canister_upgrade(canister_id, canister_path);
    let _res = request(req).await?;

    Ok(())
}

// request_cycles
pub async fn request_cycles() -> Result<(), Error> {
    // Get the schema and balance, handling potential errors early
    let canister_schema = crate::ic::canister::schema()?;
    let balance = crate::ic::canister::balance();

    log!(
        Log::Info,
        "cc check: balance: {}, initial {}, min {}",
        format_cycles(balance),
        format_cycles(canister_schema.initial_cycles),
        format_cycles(canister_schema.min_cycles)
    );

    // Check if we need cycles and calculate the needed amount
    let cycles_needed =
        if balance < canister_schema.min_cycles && canister_schema.initial_cycles > balance {
            canister_schema.initial_cycles - balance
        } else {
            0
        };

    // Request cycles if needed
    if cycles_needed > 0 {
        let req = Request::new_cycles(cycles_needed);
        let response = request(req).await?;

        match response {
            Response::Cycles => {
                log!(
                    Log::Info,
                    "cc check: requested {}, end balance: {}",
                    format_cycles(cycles_needed),
                    format_cycles(balance)
                );

                Ok(())
            }
            _ => Err(Error::InvalidResponse { response })?,
        }
    } else {
        Ok(())
    }
}
