pub mod db;
pub mod fixtures;
pub mod query;

use crate::Error;
use mimic::{
    Error as MimicError,
    schema::{
        get_schema,
        node::{Canister, Entity, Schema, Store},
    },
};
use proc_macro2::TokenStream;
use quote::quote;
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error as ThisError;

///
/// ActorError
///

#[derive(Debug, Serialize, ThisError)]
pub enum ActorError {
    #[error("canister path not found: {0}")]
    CanisterNotFound(String),

    #[error(transparent)]
    MimicError(#[from] MimicError),
}

// generate
pub fn generate(canister_path: &str) -> Result<String, Error> {
    // load schema and get the specified canister
    let schema = get_schema().map_err(ActorError::MimicError)?;

    // filter by name
    let canister = schema
        .try_get_node_as::<Canister>(canister_path)
        .map_err(|_| ActorError::CanisterNotFound(canister_path.to_string()))?;

    // create the ActorBuilder and generate the code
    let code = ActorBuilder::new(Arc::new(schema.clone()), canister.clone());
    let tokens = code.generate();

    Ok(tokens.to_string())
}

///
/// ActorBuilder
///

pub struct ActorBuilder {
    pub schema: Arc<Schema>,
    pub canister: Canister,
}

impl ActorBuilder {
    // new
    #[must_use]
    pub const fn new(schema: Arc<Schema>, canister: Canister) -> Self {
        Self { schema, canister }
    }

    // generate
    #[must_use]
    pub fn generate(self) -> TokenStream {
        let mut tokens = quote!();

        // shared between all canisters
        tokens.extend(db::generate(&self));
        tokens.extend(fixtures::generate(&self));
        tokens.extend(query::generate(&self));

        // quote
        quote! {
            #tokens
        }
    }

    // get_stores
    #[must_use]
    pub fn get_stores(&self) -> Vec<(String, Store)> {
        let canister_path = self.canister.def.path();

        self.schema
            .filter_nodes::<Store, _>(|node| node.canister == canister_path)
            .map(|(path, store)| (path.to_string(), store.clone()))
            .collect()
    }

    // get_entities
    // helper function to get all the entities for the current canister
    #[must_use]
    pub fn get_entities(&self) -> Vec<(String, Entity)> {
        let canister_path = self.canister.def.path();
        let mut entities = Vec::new();

        for (store_path, _) in self
            .schema
            .filter_nodes::<Store, _>(|node| node.canister == canister_path)
        {
            for (entity_path, entity) in self
                .schema
                .filter_nodes::<Entity, _>(|node| node.store == store_path)
            {
                entities.push((entity_path.to_string(), entity.clone()));
            }
        }

        entities
    }
}
