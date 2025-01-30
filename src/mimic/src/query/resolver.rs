use crate::{
    core::schema::{get_schema, SchemaError},
    orm::schema::node::Entity,
    store::types::DataKey,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// ResolverError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum ResolverError {
    #[snafu(display("entity not found: {path}"))]
    EntityNotFound { path: String },

    #[snafu(transparent)]
    SchemaError { source: SchemaError },
}

impl ResolverError {
    #[must_use]
    pub fn entity_not_found(path: &str) -> Self {
        Self::EntityNotFound {
            path: path.to_string(),
        }
    }
}

///
/// Resolver
///
/// generates a sort key using the Entity's path
///

pub struct Resolver {
    pub entity: String,
}

impl Resolver {
    #[must_use]
    pub fn new(entity: &str) -> Self {
        Self {
            entity: entity.to_string(),
        }
    }

    // data_key
    pub fn data_key(&self, ck: &[String]) -> Result<DataKey, ResolverError> {
        let chain_format = self.chain_format()?;

        // Initialize an empty vector to store key parts
        let mut data_key_parts: Vec<(String, Option<String>)> = Vec::new();

        for (i, part) in chain_format.iter().enumerate() {
            let key = ck.get(i).cloned();
            data_key_parts.push((part.clone(), key));
        }

        Ok(DataKey::new(data_key_parts))
    }

    // chain_format
    // returns the data used to format the sort key
    fn chain_format(&self) -> Result<Vec<String>, ResolverError> {
        let schema = get_schema()?;

        // create the chain from the Schema
        let entity = schema
            .get_node::<Entity>(&self.entity)
            .ok_or_else(|| ResolverError::entity_not_found(&self.entity))?;

        // create an ordered vec from the parents
        let mut chain = Vec::new();
        for sk in &entity.sort_keys {
            let sk_entity = schema
                .get_node::<Entity>(&sk.entity)
                .ok_or_else(|| ResolverError::entity_not_found(&sk.entity))?;

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
