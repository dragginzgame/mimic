use super::ActorBuilder;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    canister_endpoints(builder);
    cascade_endpoints(builder);
    ic_endpoints(builder);
    state_endpoints(builder);
    store_endpoints(builder);
}

// canister_endpoints
pub fn canister_endpoints(builder: &mut ActorBuilder) {
    let q = quote! {

        // canister_info
        #[::mimic::lib::ic::update]
        async fn canister_info() -> ::mimic::lib::ic::api::management_canister::main::CanisterInfoResponse {
            let req = ::mimic::lib::ic::api::management_canister::main::CanisterInfoRequest {
                canister_id: id(),
                num_requested_changes: None,
            };

            ::mimic::lib::ic::api::management_canister::main::canister_info(req)
                .await
                .unwrap()
                .0
        }

        // canister_caller
        #[::mimic::lib::ic::query]
        fn canister_caller() -> Principal {
            ::mimic::api::ic::canister::caller()
        }

        // canister_id
        #[::mimic::lib::ic::query]
        fn canister_id() -> Principal {
            ::mimic::api::ic::canister::id()
        }

        // canister_path
        #[::mimic::lib::ic::query]
        fn canister_path() -> Result<String, ::mimic::api::Error> {
            ::mimic::api::ic::canister::path().map_err(::mimic::api::Error::from)
        }

        // canister_time
        #[::mimic::lib::ic::query]
        fn canister_time() -> u64 {
            ::mimic::api::ic::canister::time()
        }

        // canister_version
        #[::mimic::lib::ic::query]
        fn canister_version() -> u64 {
            ::mimic::api::ic::canister::version()
        }

        // canister_upgrade_children
        // canister_id : None means upgrade all children
        #[::mimic::lib::ic::update]
        async fn canister_upgrade_children(
            canister_id: Option<Principal>,
        ) -> Result<(), ::mimic::api::Error> {
            guard(vec![Guard::Controller]).await?;

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
        #[::mimic::lib::ic::update]
        async fn app_state_cascade(state: AppState) -> Result<(), ::mimic::api::Error> {
            guard(vec![Guard::Parent]).await.map_err(::mimic::api::Error::from)?;

            // set state and cascade
            ::mimic::core::state::AppStateManager::set(state)?;
            ::mimic::api::subnet::cascade::app_state_cascade().await?;

            Ok(())
        }

        // subnet_index_cascade
        #[::mimic::lib::ic::update]
        async fn subnet_index_cascade(index: SubnetIndex) -> Result<(), ::mimic::api::Error> {
            guard(vec![Guard::Parent]).await?;

            // set index and cascade
            SubnetIndexManager::set(index);
            ::mimic::api::subnet::cascade::subnet_index_cascade().await?;

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
        #[::mimic::lib::ic::pre_upgrade]
        fn pre_upgrade() {
            StartupManager::pre_upgrade().unwrap();
        }

        // post_upgrade
        #[::mimic::lib::ic::post_upgrade]
        fn post_upgrade() {
            StartupManager::startup().unwrap();
            StartupManager::post_upgrade().unwrap();
        }

        ///
        /// IC API ENDPOINTS
        /// these are specific endpoints defined by the IC spec
        ///

        // ic_cycles_accept
        #[::mimic::lib::ic::update]
        fn ic_cycles_accept(max_amount: u64) -> u64 {
            ::mimic::lib::ic::api::call::msg_cycles_accept(max_amount)
        }

    };

    builder.extend_actor(q);
}

// state_endpoints
pub fn state_endpoints(builder: &mut ActorBuilder) {
    let q = quote! {

        // app_state
        #[::mimic::lib::ic::query]
        fn app_state() -> ::mimic::core::state::AppState {
            ::mimic::core::state::AppStateManager::get()
        }

        // canister_state
        #[::mimic::lib::ic::query]
        fn canister_state() -> ::mimic::core::state::CanisterState {
            ::mimic::core::state::CanisterStateManager::get()
        }

        // child_index
        #[::mimic::lib::ic::query]
        fn child_index() -> ::mimic::core::state::ChildIndex {
            ::mimic::core::state::ChildIndexManager::get()
        }

        // subnet_index
        #[::mimic::lib::ic::query]
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
        #[::mimic::lib::ic::query(composite = true)]
        #[allow(clippy::needless_pass_by_value)]
        async fn store_keys(store_name: String) -> Result<Vec<String>, ::mimic::api::Error> {
            guard(vec![Guard::Controller]).await?;

            // get keys
            let keys: Vec<String> = DB.with(|db| {
                db.with_store(&store_name, |store| {
                    Ok(store.data.keys().map(|k| k.to_string()).collect())
                })
            })
            .map_err(::mimic::db::Error::from)?;

            Ok(keys)
        }
    };

    builder.extend_actor(q);
}
