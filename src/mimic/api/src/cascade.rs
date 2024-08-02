use crate::{call, Error};
use lib_ic::{log, Log};

// app_state_cascade
pub async fn app_state_cascade() -> Result<(), Error> {
    let app_state = crate::state::app_state();

    // iterate child canisters
    for (id, path) in crate::state::child_index() {
        log!(Log::Info, "app_state_cascade: -> {id} ({path})");

        call::<_, (Result<(), Error>,)>(id, "app_state_cascade", (app_state,))
            .await?
            .0?;
    }

    Ok(())
}

// subnet_index_cascade
pub async fn subnet_index_cascade() -> Result<(), Error> {
    let subnet_index = crate::state::subnet_index();

    // iterate child canisters
    for (id, path) in crate::state::child_index() {
        log!(Log::Info, "subnet_index_cascade: -> {id} ({path})",);

        call::<_, (Result<(), Error>,)>(id, "subnet_index_cascade", (subnet_index.clone(),))
            .await?
            .0?;
    }

    Ok(())
}
