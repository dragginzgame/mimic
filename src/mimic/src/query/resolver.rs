use crate::{
    ThisError,
    db::types::SortKey,
    schema::{
        node::Entity,
        state::{StateError as SchemaStateError, get_schema},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// ResolverError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum ResolverError {
    #[error("entity not found: {0}")]
    EntityNotFound(String),

    #[error(transparent)]
    SchemaStateError(#[from] SchemaStateError),
}

///
/// Resolver
///
/// generates a sort key using the Entity's path
///

pub struct Resolver {
    pub path: String,
}

impl Resolver {
    #[must_use]
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    // store
    pub fn store(&self) -> Result<String, ResolverError> {
        let schema = get_schema()?;
        let entity = schema
            .get_node_as::<Entity>(&self.path)
            .ok_or_else(|| ResolverError::EntityNotFound(self.path.clone()))?;

        Ok(entity.store.clone())
    }

    // data_key
    pub fn data_key(&self, ck: &[String]) -> Result<SortKey, ResolverError> {
        let chain_format = self.chain_format()?;

        // Initialize an empty vector to store key parts
        let mut data_key_parts: Vec<(String, Option<String>)> = Vec::new();

        for (i, part) in chain_format.iter().enumerate() {
            let key = ck.get(i).cloned();
            data_key_parts.push((part.clone(), key));
        }

        Ok(SortKey::new(data_key_parts))
    }

    // chain_format
    // returns the data used to format the sort key
    fn chain_format(&self) -> Result<Vec<String>, ResolverError> {
        let schema = get_schema().map_err(ResolverError::SchemaStateError)?;

        // create the chain from the Schema
        let entity = schema
            .get_node_as::<Entity>(&self.path)
            .ok_or_else(|| ResolverError::EntityNotFound(self.path.clone()))?;

        // create an ordered vec from the parents
        let mut chain = Vec::new();
        for sk in &entity.sort_keys {
            let sk_entity = schema
                .get_node_as::<Entity>(&sk.entity)
                .ok_or_else(|| ResolverError::EntityNotFound(sk.entity.clone()))?;

            chain.push(sk_entity);
        }

        //
        // format the chain
        //

        let mut format = Vec::new();
        for (i, entity) in chain.into_iter().enumerate() {
            let part = if i == 0 {
                entity.def.path()
            } else {
                entity.def.ident.to_string()
            };

            format.push(part);
        }

        Ok(format)
    }
}
