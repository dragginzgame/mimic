pub mod db;
pub mod fixtures;

use crate::Error;
use mimic::{
    schema::{
        get_schema,
        node::{Canister, Entity, Store},
    },
    Error as MimicError,
};
use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// ActorError
///

#[derive(Debug, Serialize, Deserialize, ThisError)]
pub enum ActorError {
    #[error("canister path not found: {0}")]
    CanisterNotFound(String),

    #[error(transparent)]
    MimicError(#[from] MimicError),
}

// generate
pub fn generate(canister_path: &str) -> Result<String, Error> {
    // load schema and get the specified canister
    let schema = mimic::schema::get_schema().map_err(ActorError::MimicError)?;

    // filter by name
    let canister = schema
        .try_get_node_as::<Canister>(canister_path)
        .map_err(|_| ActorError::CanisterNotFound(canister_path.to_string()))?;

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
    pub tokens: TokenStream,
}

impl ActorBuilder {
    // new
    #[must_use]
    pub fn new(canister: Canister) -> Self {
        Self {
            canister,
            init_hooks: Vec::new(),
            tokens: quote!(),
        }
    }

    // extend_actor
    pub fn extend(&mut self, tokens: TokenStream) {
        self.tokens.extend(tokens);
    }

    // expand
    #[must_use]
    pub fn expand(mut self) -> TokenStream {
        //
        // shared between all crates
        //

        fixtures::extend(&mut self);
        db::extend(&mut self);

        //
        // generate code
        //

        let tokens = &self.tokens;
        quote! {
            #tokens
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
