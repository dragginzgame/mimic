use crate::{
    api::ic::{
        call::{call, CallError},
        canister::CanisterError,
        create::CreateError,
        mgmt::MgmtError,
        upgrade::UpgradeError,
    },
    core::{
        state::ChildIndexManager,
        wasm::{WasmError, WasmManager},
    },
    ic::{caller, format_cycles, println},
    log, Error, Log,
};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use strum::Display;

///
/// RequestError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum RequestError {
    #[snafu(display("invalid response: {response}"))]
    InvalidResponse { response: Response },

    #[snafu(transparent)]
    Error { source: Error },

    #[snafu(transparent)]
    CallError { source: CallError },

    #[snafu(transparent)]
    CanisterError { source: CanisterError },

    #[snafu(transparent)]
    CreateError { source: CreateError },

    #[snafu(transparent)]
    MgmtError { source: MgmtError },

    #[snafu(transparent)]
    UpgradeError { source: UpgradeError },

    #[snafu(transparent)]
    WasmError { source: WasmError },
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
pub async fn response(req: Request) -> Result<Response, RequestError> {
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
async fn response_create_canister(path: &str) -> Result<Response, RequestError> {
    let bytes = WasmManager::get_wasm(path)?;
    let new_canister_id = crate::api::ic::create::create_canister(path, bytes, caller()).await?;

    Ok(Response::CanisterCreate(new_canister_id))
}

// response_upgrade_canister
async fn response_upgrade_canister(
    canister_id: Principal,
    path: &str,
) -> Result<Response, RequestError> {
    let bytes = WasmManager::get_wasm(path)?;
    crate::api::ic::upgrade::upgrade_canister(canister_id, bytes).await?;

    Ok(Response::CanisterUpgrade)
}

// response_send_cycles
async fn response_send_cycles(
    canister_id: Principal,
    cycles: u128,
) -> Result<Response, RequestError> {
    // actually send cycles
    crate::api::ic::mgmt::deposit_cycles(canister_id, cycles).await?;

    // debug
    let balance = crate::api::ic::canister::balance();
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
pub async fn request(request: Request) -> Result<Response, RequestError> {
    println!("request: {request:?}");

    let root_canister_id = crate::api::ic::canister::root_id()?;
    let res = call::<_, (Result<Response, Error>,)>(root_canister_id, "response", (request,))
        .await?
        .0?;

    Ok(res)
}

// request_canister_create
// create a Request and pass it to the request shared endpoint
pub async fn request_canister_create(canister_path: &str) -> Result<Principal, RequestError> {
    let req = Request::new_canister_create(canister_path.to_string());

    match request(req).await {
        Ok(response) => match response {
            Response::CanisterCreate(new_canister_id) => {
                // success, update child index
                ChildIndexManager::add_canister(new_canister_id, canister_path);

                Ok(new_canister_id)
            }
            _ => Err(RequestError::InvalidResponse { response })?,
        },
        Err(e) => Err(e),
    }
}

// request_canister_upgrade
pub async fn request_canister_upgrade(
    canister_id: Principal,
    canister_path: String,
) -> Result<(), RequestError> {
    let req = Request::new_canister_upgrade(canister_id, canister_path);
    let _res = request(req).await?;

    Ok(())
}

// request_cycles
pub async fn request_cycles() -> Result<(), RequestError> {
    // Get the schema and balance, handling potential errors early
    let canister_schema = crate::api::ic::canister::schema()?;
    let balance = crate::api::ic::canister::balance();

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
            _ => Err(RequestError::InvalidResponse { response })?,
        }
    } else {
        Ok(())
    }
}
