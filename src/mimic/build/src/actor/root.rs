use super::ActorBuilder;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    root_actor(builder);
    root_module(builder);
}

// root_actor
pub fn root_actor(builder: &mut ActorBuilder) {
    let q = quote! {
        // app
        // modify app-level state
        // @todo eventually this will cascade down from an orchestrator canister
        #[::mimic::ic::update]
        async fn app(cmd: AppCommand) -> Result<(), ::mimic::api::Error> {
            AppStateManager::command(cmd)?;

            ::mimic::api::cascade::app_state_cascade().await?;

            Ok(())
        }

        // response
        #[::mimic::ic::update]
        async fn response(req: Request) -> Result<Response, ::mimic::api::Error> {
            let res = ::mimic::api::request::response(req).await?;

            Ok(res)
        }
    };

    builder.extend_actor(q);
}

// root_module
pub fn root_module(builder: &mut ActorBuilder) {
    let q = quote! {
        // root_auto_create_canisters
        pub async fn root_auto_create_canisters() -> Result<(), ::mimic::api::Error> {
            use ::mimic::orm::schema::node::Canister;

            guard(vec![Guard::Controller]).await?;

            // Collect all service canister paths directly into a vector of tokens.
            let schema = ::mimic::api::schema::get_schema()?;
            let paths: Vec<_> = schema
                .filter_nodes::<Canister, _>(|node| node.build.is_auto_created())
                .map(|(key, _)| key)
                .collect();

            for path in paths {
                if SubnetIndexManager::get_canister(path).is_none() {
                    // set the canister within the service index
                    let new_canister_id = ::mimic::api::request::request_canister_create(path)
                        .await?;

                    SubnetIndexManager::set_canister(path, new_canister_id);
                } else {
                    log!(
                        Log::Warn,
                        "auto_create_canisters: canister {path} already exists"
                    );
                }
            }

            // cascade subnet_index
            ::mimic::api::cascade::subnet_index_cascade().await?;

            Ok(())
        }
    };

    builder.extend_module(q);
}
