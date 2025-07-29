use crate::{
    Error,
    core::Key,
    db::query::{DeleteQuery, SaveQuery},
    interface::InterfaceError,
};
use candid::Principal;
use icu::ic::call::Call;
use thiserror::Error as ThisError;

///
/// QueryError
///

#[derive(Debug, ThisError)]
pub enum QueryError {
    #[error("call error: {0}")]
    CallError(String),

    #[error("entity not found: {0}")]
    EntityNotFound(String),
}

// query_load
// currently disabled because LoadResponse needs a rethink
//pub async fn query_load(pid: Principal, query: LoadQuery) -> Result<LoadResponse, MimicError> {
//    query_call(pid, "mimic_query_load", &query).await
//}

// query_save
pub async fn query_save(pid: Principal, query: SaveQuery) -> Result<Key, Error> {
    query_call(pid, "mimic_query_save", &query).await
}

// query_delete
pub async fn query_delete(pid: Principal, query: DeleteQuery) -> Result<Vec<Key>, Error> {
    query_call(pid, "mimic_query_delete", &query).await
}

// query_call
// private helper method
async fn query_call<T: candid::CandidType + for<'de> candid::Deserialize<'de>>(
    pid: Principal,
    method: &str,
    arg: impl candid::CandidType,
) -> Result<T, Error> {
    let result = Call::unbounded_wait(pid, method)
        .with_arg(arg)
        .await
        .map_err(|e| QueryError::CallError(e.to_string()))
        .map_err(InterfaceError::QueryError)?;

    let response = result
        .candid::<T>()
        .map_err(|e| QueryError::CallError(e.to_string()))
        .map_err(InterfaceError::QueryError)?;

    Ok(response)
}
