pub mod crud;
pub mod endpoints;
pub mod fixtures;
pub mod init;
pub mod root;
pub mod stores;
pub mod timers;
pub mod user;

use clap::Parser;
use orm_schema::{
    build::schema,
    node::{Canister, CanisterBuild, Entity, Store},
};
use proc_macro2::TokenStream;
use quote::quote;
use std::process;

///
/// Command
///

#[derive(Parser)]
pub struct Command {
    #[clap(help = "Name of the canister to generate code for")]
    canister_name: String,
}

// process
pub fn process(command: Command) {
    // load schema and get the specified canister
    let schema = schema();
    let mut canisters =
        schema.filter_nodes::<Canister, _>(|node| node.name() == command.canister_name);
    let Some((_, canister)) = canisters.next() else {
        eprintln!(
            "Canister '{}' not found in the schema",
            command.canister_name
        );
        process::exit(1);
    };

    // create the ActorBuilder and generate the code
    let code = ActorBuilder::new(canister.clone());
    let tokens = code.expand();

    println!("{tokens}");
}

///
/// ActorBuilder
///

pub struct ActorBuilder {
    pub canister: Canister,
    pub hooks: Vec<String>,
    pub actor_tokens: TokenStream,
    pub module_tokens: TokenStream,
}

impl ActorBuilder {
    // new
    #[must_use]
    pub fn new(canister: Canister) -> Self {
        Self {
            canister,
            hooks: Vec::new(),
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

    // add_hook
    pub fn add_hook(&mut self, hook: &str) {
        self.hooks.push(hook.to_string());
    }

    // expand
    #[must_use]
    pub fn expand(mut self) -> TokenStream {
        // all get these
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
        self.add_hook("init2");
        init::extend(&mut self);

        //
        // generate code
        //

        let actor_tokens = self.actor_tokens;
        let module_tokens = self.module_tokens;
        quote! {

            // load config
        //    let config_str = include_str!("../../../config.toml");
        //    ::mimic::config::init_toml(config_str).expect("Failed to load configuration");

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

        for (store_path, store) in
            schema().filter_nodes::<Store, _>(|node| node.canister == canister_path)
        {
            stores.push((store_path.to_string(), store.clone()));
        }

        stores
    }

    // get_entities
    // helper function to get all the entities for the current canister
    #[must_use]
    pub fn get_entities(&self) -> Vec<(String, Entity)> {
        let schema = schema();
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
