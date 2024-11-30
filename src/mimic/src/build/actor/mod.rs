pub mod crud;
pub mod endpoints;
pub mod fixtures;
pub mod init;
pub mod root;
pub mod shared;
pub mod stores;
pub mod timers;
pub mod user;

use crate::orm::schema::{
    build::get_schema,
    node::{Canister, CanisterBuild, Entity, Store},
};
use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("canister '{canister}' not found in schema"))]
    CanisterNotFound { canister: String },

    #[snafu(transparent)]
    Schema {
        source: crate::orm::schema::build::Error,
    },
}

// generate
pub fn generate(canister_name: &str) -> Result<String, Error> {
    // load schema and get the specified canister
    let schema = get_schema()?;
    let mut canisters = schema.filter_nodes::<Canister, _>(|node| node.name() == canister_name);
    let Some((_, canister)) = canisters.next() else {
        return Err(Error::CanisterNotFound {
            canister: canister_name.to_string(),
        });
    };

    // create the ActorBuilder and generate the code
    let code = ActorBuilder::new(canister.clone());
    let tokens = code.expand();

    Ok(tokens.to_string())
}

///
/// ActorBuilder
///

pub struct ActorBuilder {
    pub canister: Canister,
    pub init_hooks: Vec<String>,
    pub actor_tokens: TokenStream,
    pub module_tokens: TokenStream,
}

impl ActorBuilder {
    // new
    #[must_use]
    pub fn new(canister: Canister) -> Self {
        Self {
            canister,
            init_hooks: Vec::new(),
            actor_tokens: quote!(),
            module_tokens: quote!(),
        }
    }

    // extend_actor
    pub fn extend_actor(&mut self, tokens: TokenStream) {
        self.actor_tokens.extend(tokens);
    }

    // extend_module
    pub fn extend_module(&mut self, tokens: TokenStream) {
        self.module_tokens.extend(tokens);
    }

    // add_init_hook
    pub fn add_init_hook(&mut self, hook: &str) {
        self.init_hooks.push(hook.to_string());
    }

    // expand
    #[must_use]
    pub fn expand(mut self) -> TokenStream {
        // all get these
        shared::extend(&mut self);

        endpoints::extend(&mut self);
        crud::extend(&mut self);
        fixtures::extend(&mut self);
        stores::extend(&mut self);
        timers::extend(&mut self);

        // root
        match &self.canister.build {
            CanisterBuild::Root => {
                root::extend(&mut self);
            }
            CanisterBuild::User => {
                user::extend(&mut self);
            }
            _ => {}
        }

        // init
        // this goes last because it has registered hooks
        self.add_init_hook("StartupManager::init");
        init::extend(&mut self);

        //
        // generate code
        //

        let actor_tokens = self.actor_tokens;
        let module_tokens = self.module_tokens;
        quote! {
            #actor_tokens

            #[allow(clippy::wildcard_imports)]
            pub mod actorgen {
                use super::*;

                #module_tokens
            }
        }
    }

    // get_stores
    #[must_use]
    pub fn get_stores(&self) -> Vec<(String, Store)> {
        let canister_path = self.canister.def.path();
        let mut stores = Vec::new();

        for (store_path, store) in get_schema()
            .unwrap()
            .filter_nodes::<Store, _>(|node| node.canister == canister_path)
        {
            stores.push((store_path.to_string(), store.clone()));
        }

        stores
    }

    // get_entities
    // helper function to get all the entities for the current canister
    #[must_use]
    pub fn get_entities(&self) -> Vec<(String, Entity)> {
        let schema = get_schema().unwrap();
        let canister_path = self.canister.def.path();
        let mut entities = Vec::new();

        for (store_path, _) in
            schema.filter_nodes::<Store, _>(|node| node.canister == canister_path)
        {
            for (entity_path, entity) in
                schema.filter_nodes::<Entity, _>(|node| node.store == store_path)
            {
                entities.push((entity_path.to_string(), entity.clone()));
            }
        }

        entities
    }
}
