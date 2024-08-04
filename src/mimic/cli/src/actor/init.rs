use super::ActorBuilder;
use orm_schema::node::CanisterBuild;
use proc_macro2::TokenStream;
use syn::{parse_str, Path};
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    let q = match &builder.canister.build {
        CanisterBuild::Root => init_root(builder),
        CanisterBuild::Test => init_test(builder),
        CanisterBuild::Basic(_) | CanisterBuild::User => init_default(builder),
    };

    builder.extend_actor(q);
}

// init_root
fn init_root(builder: &ActorBuilder) -> TokenStream {
    let hooks = format_hooks(&builder.hooks);
    let canister_path = builder.canister.def.path();

    quote! {
        #[::mimic::ic::init]
        fn init() {
            let id = id();

            log!(Log::Info, "**********************************");
            log!(Log::Info, "init: root");
            log!(Log::Info, "**********************************");

            CanisterStateManager::set_path(#canister_path.to_string()).unwrap();
            CanisterStateManager::set_root_id(id).unwrap();

            #hooks
        }
    }
}

// init_default
fn init_default(builder: &ActorBuilder) -> TokenStream {
    let hooks = format_hooks(&builder.hooks);
    let canister_path = builder.canister.def.path();

    quote! {
        #[::mimic::ic::init]
        fn init(root_id: Principal, parent_id: Principal) {
            log!(Log::Info, "init: {}", #canister_path);

            CanisterStateManager::set_path(#canister_path.to_string()).unwrap();
            CanisterStateManager::set_root_id(root_id).unwrap();
            CanisterStateManager::set_parent_id(parent_id).unwrap();

            #hooks
        }
    }
}

// init_test
fn init_test(builder: &ActorBuilder) -> TokenStream {
    let hooks = format_hooks(&builder.hooks);
    let canister_path = builder.canister.def.path();

    quote! {
        #[::mimic::ic::init]
        fn init() {
            log!(Log::Info, "init: test");

            CanisterStateManager::set_path(#canister_path.to_string()).unwrap();

            #hooks
        }
    }
}

// format_hooks
fn format_hooks(hooks: &[String]) -> TokenStream {
    let hook_calls: Vec<TokenStream> = hooks
        .iter()
        .map(|hook| {
            // Parse as path because it could have a module
            let hook_path: Path = parse_str(hook).expect("Failed to parse hook path");
            quote! {
                #hook_path().unwrap();
            }
        })
        .collect();

    quote! {
        #(#hook_calls)*
    }
}
