use super::ActorBuilder;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    canister_endpoints(builder);
    cascade_endpoints(builder);
    db_endpoints(builder);
    ic_endpoints(builder);
    state_endpoints(builder);
    store_endpoints(builder);
}

// canister_endpoints
pub fn canister_endpoints(builder: &mut ActorBuilder) {
    let q = quote! {

        // canister_info
        #[::mimic::ic::update(guard = "guard_update")]
        async fn canister_info() -> ::mimic::ic::api::management_canister::main::CanisterInfoResponse {
            let req = ::mimic::ic::api::management_canister::main::CanisterInfoRequest {
                canister_id: id(),
                num_requested_changes: None,
            };

            ::mimic::ic::api::management_canister::main::canister_info(req)
                .await
                .unwrap()
                .0
        }

        // canister_caller
        #[::mimic::ic::query]
        fn canister_caller() -> Principal {
            ::mimic::api::ic::canister::caller()
        }

        // canister_id
        #[::mimic::ic::query]
        fn canister_id() -> Principal {
            ::mimic::api::ic::canister::id()
        }

        // canister_path
        #[::mimic::ic::query]
        fn canister_path() -> Result<String, ::mimic::api::Error> {
            ::mimic::api::ic::canister::path().map_err(::mimic::api::Error::from)
        }

        // canister_time
        #[::mimic::ic::query]
        fn canister_time() -> u64 {
            ::mimic::api::ic::canister::time()
        }

        // canister_version
        #[::mimic::ic::query]
        fn canister_version() -> u64 {
            ::mimic::api::ic::canister::version()
        }

        // canister_upgrade_children
        // canister_id : None means upgrade all children
        #[::mimic::ic::update(guard = "guard_update")]
        async fn canister_upgrade_children(
            canister_id: Option<Principal>,
        ) -> Result<(), ::mimic::api::Error> {
            allow_any(vec![Auth::Controller]).await?;

            // send a request for each matching canister
            for (child_id, path) in child_index() {
                if canister_id.is_none() || canister_id == Some(child_id) {
                    if let Err(e) =
                        ::mimic::api::subnet::request::request_canister_upgrade(child_id, path.clone()).await
                    {
                        log!(Log::Warn, "{child_id} ({path}): {e}");
                    }
                }
            }

            Ok(())
        }
    };

    builder.extend_actor(q);
}

// cascade_endpoints
pub fn cascade_endpoints(builder: &mut ActorBuilder) {
    let q = quote! {

        // app_state_cascade
        // NO guard because this is set from the parent
        #[::mimic::ic::update]
        async fn app_state_cascade(state: AppState) -> Result<(), ::mimic::api::Error> {
            allow_any(vec![Auth::Parent]).await.map_err(::mimic::api::Error::from)?;

            // set state and cascade
            ::mimic::core::state::AppStateManager::set(state)?;
            ::mimic::api::subnet::cascade::app_state_cascade().await?;

            Ok(())
        }

        // subnet_index_cascade
        // NO guard because this is set from the parent
        #[::mimic::ic::update]
        async fn subnet_index_cascade(index: SubnetIndex) -> Result<(), ::mimic::api::Error> {
            allow_any(vec![Auth::Parent]).await?;

            // set index and cascade
            SubnetIndexManager::set(index);
            ::mimic::api::subnet::cascade::subnet_index_cascade().await?;

            Ok(())
        }
    };

    builder.extend_actor(q);
}

// db_endpoints
pub fn db_endpoints(builder: &mut ActorBuilder) {
    let q = quote! {

        #[::mimic::ic::query(guard = "guard_query")]
        async fn db_load() -> Result<(), ::mimic::api::Error> {
            allow_any(vec![Auth::Parent]).await.map_err(::mimic::api::Error::from)?;

            Ok(())
        }

    };

    builder.extend_actor(q);
}

// ic_endpoints
pub fn ic_endpoints(builder: &mut ActorBuilder) {
    let q = quote! {

        ///
        /// LIFECYCLE ENDPOINTS
        ///

        // pre_upgrade
        // be careful here because this can brick a canister
        #[::mimic::ic::pre_upgrade]
        fn pre_upgrade() {
            StartupManager::pre_upgrade().unwrap();
        }

        // post_upgrade
        #[::mimic::ic::post_upgrade]
        fn post_upgrade() {
            log!(Log::Info, "post_upgrade()");

            // post_upgrade is considered a startup along with init
            // this will also call the StartupManager::startup
            startup().unwrap();

            StartupManager::post_upgrade().unwrap();
        }

        ///
        /// IC API ENDPOINTS
        /// these are specific endpoints defined by the IC spec
        ///

        // ic_cycles_accept
        #[::mimic::ic::update]
        fn ic_cycles_accept(max_amount: u64) -> u64 {
            ::mimic::ic::api::call::msg_cycles_accept(max_amount)
        }
    };

    builder.extend_actor(q);
}

// state_endpoints
pub fn state_endpoints(builder: &mut ActorBuilder) {
    let q = quote! {

        // app_state
        #[::mimic::ic::query]
        fn app_state() -> ::mimic::core::state::AppState {
            ::mimic::core::state::AppStateManager::get()
        }

        // canister_state
        #[::mimic::ic::query]
        fn canister_state() -> ::mimic::core::state::CanisterState {
            ::mimic::core::state::CanisterStateManager::get()
        }

        // child_index
        #[::mimic::ic::query]
        fn child_index() -> ::mimic::core::state::ChildIndex {
            ::mimic::core::state::ChildIndexManager::get()
        }

        // subnet_index
        #[::mimic::ic::query]
        fn subnet_index() -> ::mimic::core::state::SubnetIndex {
            ::mimic::core::state::SubnetIndexManager::get()
        }
    };

    builder.extend_actor(q);
}

// store_endpoints
pub fn store_endpoints(builder: &mut ActorBuilder) {
    let q = quote! {

        // store_keys
        #[::mimic::ic::query(guard = "guard_query", composite = true)]
        #[allow(clippy::needless_pass_by_value)]
        async fn store_keys(store_path: String) -> Result<Vec<String>, ::mimic::api::Error> {
            allow_any(vec![Auth::Controller]).await?;

            // get keys
            let keys: Vec<String> = DB.with(|db| {
                db.with_store(&store_path, |store| {
                    Ok(store.data.keys().map(|k| k.to_string()).collect())
                })
            })
            .map_err(::mimic::db::Error::from)?;

            Ok(keys)
        }

        // store_clear
        #[::mimic::ic::update(guard = "guard_update")]
        #[allow(clippy::needless_pass_by_value)]
        async fn store_clear(store_path: String) -> Result<(), ::mimic::api::Error> {
            allow_any(vec![Auth::Controller]).await?;

            // clear canister
            DB.with(|db| {
                db.with_store_mut(&store_path, |store| {
                    Ok(store.clear())
                })
            })
            .map_err(::mimic::db::Error::from)?;

            Ok(())
        }
    };

    builder.extend_actor(q);
}
