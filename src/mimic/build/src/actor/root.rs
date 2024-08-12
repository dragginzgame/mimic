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
        async fn app(cmd: AppCommand) -> Result<(), ::mimic::Error> {
            AppStateManager::command(cmd)?;

            ::mimic::api::cascade::app_state_cascade().await?;

            Ok(())
        }

        // response
        #[::mimic::ic::update]
        async fn response(req: Request) -> Result<Response, ::mimic::Error> {
            let res = ::mimic::api::request::response(req).await?;

            Ok(res)
        }

        // schema
        #[::mimic::ic::query]
        async fn schema() -> Result<String, ::mimic::Error> {
            let schema = schema_read();
            let output = serde_json::to_string(schema)?;

            Ok(output)
        }
    };

    builder.extend_actor(q);
}

// root_module
pub fn root_module(builder: &mut ActorBuilder) {
    let q = quote! {
        // root_auto_create_canisters
        pub async fn root_auto_create_canisters() -> Result<(), ::mimic::Error> {
            use ::mimic::orm::schema::node::Canister;

            guard(vec![Guard::Controller]).await?;

            // Collect all service canister paths directly into a vector of tokens.
            let schema = ::mimic::core::schema::get_schema()?;
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
