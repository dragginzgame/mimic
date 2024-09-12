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
        #[::mimic::ic::update]
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
            ::mimic::api::canister::caller()
        }

        // canister_id
        #[::mimic::ic::query]
        fn canister_id() -> Principal {
            ::mimic::api::canister::id()
        }

        // canister_path
        #[::mimic::ic::query]
        fn canister_path() -> Result<String, ::mimic::api::Error> {
            ::mimic::api::canister::path().map_err(::mimic::api::Error::from)
        }

        // canister_time
        #[::mimic::ic::query]
        fn canister_time() -> u64 {
            ::mimic::api::canister::time()
        }

        // canister_version
        #[::mimic::ic::query]
        fn canister_version() -> u64 {
            ::mimic::api::canister::version()
        }

        // canister_upgrade_children
        // canister_id : None means upgrade all children
        #[::mimic::ic::update]
        async fn canister_upgrade_children(
            canister_id: Option<Principal>,
        ) -> Result<(), ::mimic::api::Error> {
            guard(vec![Guard::Controller]).await?;

            // send a request for each matching canister
            for (child_id, path) in child_index() {
                if canister_id.is_none() || canister_id == Some(child_id) {
                    if let Err(e) =
                        ::mimic::api::request::request_canister_upgrade(child_id, path.clone()).await
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
        #[::mimic::ic::update]
        async fn app_state_cascade(state: AppState) -> Result<(), ::mimic::api::Error> {
            guard(vec![Guard::Parent]).await.map_err(::mimic::api::Error::from)?;

            // set state and cascade
            ::mimic::api::state::app_state().set(state)?;
            ::mimic::api::cascade::app_state_cascade().await?;

            Ok(())
        }

        // subnet_index_cascade
        #[::mimic::ic::update]
        async fn subnet_index_cascade(index: SubnetIndex) -> Result<(), ::mimic::api::Error> {
            guard(vec![Guard::Parent]).await?;

            // set index and cascade
            SubnetIndexManager::set(index);
            ::mimic::api::cascade::subnet_index_cascade().await?;

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
            pre_upgrade2().unwrap();
        }

        // post_upgrade
        #[::mimic::ic::post_upgrade]
        fn post_upgrade() {
            startup().unwrap();
            post_upgrade2().unwrap();
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
            ::mimic::api::state::app_state()
        }

        // canister_state
        #[::mimic::ic::query]
        fn canister_state() -> ::mimic::core::state::CanisterState {
            ::mimic::api::state::canister_state()
        }

        // child_index
        #[::mimic::ic::query]
        fn child_index() -> ::mimic::core::state::ChildIndex {
            ::mimic::api::state::child_index()
        }

        // subnet_index
        #[::mimic::ic::query]
        fn subnet_index() -> ::mimic::core::state::SubnetIndex {
            ::mimic::api::state::subnet_index()
        }
    };

    builder.extend_actor(q);
}

// store_endpoints
pub fn store_endpoints(builder: &mut ActorBuilder) {
    let q = quote! {

        // store_keys
        #[::mimic::ic::query(composite = true)]
        #[allow(clippy::needless_pass_by_value)]
        async fn store_keys(store_name: String) -> Result<Vec<String>, ::mimic::api::Error> {
            guard(vec![Guard::Controller]).await?;

            // get keys
            let keys: Vec<String> = DB.with(|db| {
                db.with_store(&store_name, |store| {
                    Ok(store.data.keys().map(|k| k.to_string()).collect())
                })
            }).map_err(::mimic::api::Error::from)?;

            Ok(keys)
        }
    };

    builder.extend_actor(q);
}
