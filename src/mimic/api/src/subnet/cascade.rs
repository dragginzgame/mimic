use crate::ic::call::call;
use core_state::{AppStateManager, ChildIndexManager, SubnetIndexManager};
use ic::{log, Log};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("api error: {error}"))]
    Api { error: crate::Error },

    #[snafu(transparent)]
    Call { source: crate::ic::call::Error },
}

impl From<crate::Error> for Error {
    fn from(error: crate::Error) -> Self {
        Self::Api { error }
    }
}

// app_state_cascade
pub async fn app_state_cascade() -> Result<(), Error> {
    let app_state = AppStateManager::get();
    let child_index = ChildIndexManager::get();

    // iterate child canisters
    for (id, path) in child_index {
        log!(Log::Info, "app_state_cascade: -> {id} ({path})");

        call::<_, (Result<(), crate::Error>,)>(id, "app_state_cascade", (app_state,))
            .await?
            .0?;
    }

    Ok(())
}

// subnet_index_cascade
pub async fn subnet_index_cascade() -> Result<(), Error> {
    let subnet_index = SubnetIndexManager::get();
    let child_index = ChildIndexManager::get();

    // iterate child canisters
    for (id, path) in child_index {
        log!(Log::Info, "subnet_index_cascade: -> {id} ({path})",);

        call::<_, (Result<(), crate::Error>,)>(id, "subnet_index_cascade", (subnet_index.clone(),))
            .await?
            .0?;
    }

    Ok(())
}
