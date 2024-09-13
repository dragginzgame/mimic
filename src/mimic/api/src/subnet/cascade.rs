use crate::ic::call::call;
use lib_ic::{log, Log};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Call { source: crate::ic::call::Error },
}

// app_state_cascade
pub async fn app_state_cascade() -> Result<(), Error> {
    let app_state = crate::core::state::app_state();

    // iterate child canisters
    for (id, path) in crate::core::state::child_index() {
        log!(Log::Info, "app_state_cascade: -> {id} ({path})");

        call::<_, (Result<(), crate::ic::call::Error>,)>(id, "app_state_cascade", (app_state,))
            .await?
            .0?;
    }

    Ok(())
}

// subnet_index_cascade
pub async fn subnet_index_cascade() -> Result<(), Error> {
    let subnet_index = crate::core::state::subnet_index();

    // iterate child canisters
    for (id, path) in crate::core::state::child_index() {
        log!(Log::Info, "subnet_index_cascade: -> {id} ({path})",);

        call::<_, (Result<(), crate::ic::call::Error>,)>(
            id,
            "subnet_index_cascade",
            (subnet_index.clone(),),
        )
        .await?
        .0?;
    }

    Ok(())
}
