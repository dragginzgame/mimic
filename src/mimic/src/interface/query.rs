use crate::{
    Error,
    ic::call::Call,
    interface::InterfaceError,
    query::{DeleteQuery, DeleteResponse, LoadQuery, LoadResponse, SaveQuery, SaveResponse},
};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// QueryError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum QueryError {
    #[error("call error: {0}")]
    CallError(String),
}

// query_load
// has to return the dynamic response as the generic one is E
pub async fn query_load(canister_pid: Principal, query: LoadQuery) -> Result<LoadResponse, Error> {
    let result = Call::unbounded_wait(canister_pid, "query_load")
        .with_arg(&query)
        .await
        .map_err(|e| QueryError::CallError(e.to_string()))
        .map_err(InterfaceError::QueryError)?;

    let response = result
        .candid::<LoadResponse>()
        .map_err(|e| QueryError::CallError(e.to_string()))
        .map_err(InterfaceError::QueryError)?;

    Ok(response)
}

// query_delete
pub async fn query_delete(
    canister_pid: Principal,
    query: DeleteQuery,
) -> Result<DeleteResponse, Error> {
    let result = Call::unbounded_wait(canister_pid, "query_delete")
        .with_arg(&query)
        .await
        .map_err(|e| QueryError::CallError(e.to_string()))
        .map_err(InterfaceError::QueryError)?;

    let response = result
        .candid::<DeleteResponse>()
        .map_err(|e| QueryError::CallError(e.to_string()))
        .map_err(InterfaceError::QueryError)?;

    Ok(response)
}

// query_save
pub async fn query_save(canister_pid: Principal, query: SaveQuery) -> Result<SaveResponse, Error> {
    let result = Call::unbounded_wait(canister_pid, "query_save")
        .with_arg(&query)
        .await
        .map_err(|e| QueryError::CallError(e.to_string()))
        .map_err(InterfaceError::QueryError)?;

    let response = result
        .candid::<SaveResponse>()
        .map_err(|e| QueryError::CallError(e.to_string()))
        .map_err(InterfaceError::QueryError)?;

    Ok(response)
}
